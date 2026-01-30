<script lang="ts">
  /* Allenatore — landing profilo; dashboard 2026: hero + stat tiles + card. */
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { listen } from "@tauri-apps/api/event";
  import { activeProfile, loadProfiles } from "$lib/stores/profile";
  import * as profileService from "$lib/services/profile";
  import * as pokedexService from "$lib/services/pokedex";
  import { getSavEntries, getTrainerData } from "$lib/services/sav";
  import { POKEDEX_GEN4_MAX } from "$lib/data/pokedex-species";
  import { getAvatarSrc, getProfileAvatarId, isAvatarId, type AvatarId } from "$lib/data/avatars";
  import { reportSystemError } from "$lib/stores/error-archive";
  import { toast } from "$lib/components/ui/sonner";
  import { Card, CardHeader, CardTitle, CardContent } from "$lib/components/ui/card";
  import { StatCard } from "$lib/components/ui/stat-card";
  import AvatarPicker from "$lib/components/profile/AvatarPicker.svelte";
  import { watchedCount } from "$lib/stores/sync.svelte";
  import { Award, CheckCircle, Clock, Coins, Gamepad2, History, User } from "@lucide/svelte";

  /** Catturati (status === "caught") per Pokédex. */
  let pokedexCaught = $state(0);
  /** Numero giochi diversi (non versioni). */
  let gamesCount = $state(0);
  /** Ultimo gioco giocato (game con last_played_at/mtime più recente, altrimenti updated_at). */
  let lastGame = $state<string>("—");
  /** Denaro (da trainer data del save più recente). */
  let money = $state<number | null>(null);
  /** Tempo di gioco: ore (da trainer data del save più recente). */
  let playedHours = $state<number | null>(null);
  /** Tempo di gioco: minuti (da trainer data del save più recente). */
  let playedMinutes = $state<number | null>(null);
  /** Dialog AvatarPicker aperto. */
  let avatarPickerOpen = $state(false);
  /** Aggiornamento avatar in corso. */
  let avatarUpdating = $state(false);

  /** Profilo attivo (sync da store per reattività Svelte 5). */
  let profile = $state(get(activeProfile));
  const avatarId = $derived(getProfileAvatarId(profile));
  $effect(() => {
    const unsub = activeProfile.subscribe((p) => {
      profile = p;
    });
    return unsub;
  });

  async function onAvatarSelect(id: AvatarId | null) {
    const p = get(activeProfile);
    // #region agent log
    fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'profilo+page:onAvatarSelect',message:'entry',data:{profileId:p?.id,avatarId:id,hasProfile:!!p,avatarUpdating},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'H4'})}).catch(()=>{});
    // #endregion
    if (!p || avatarUpdating) return;
    avatarUpdating = true;
    try {
      await profileService.updateProfileAvatar(p.id, id);
      await loadProfiles();
      toast.success("Avatar aggiornato.");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      // #region agent log
      fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'profilo+page:catch',message:'avatar update failed',data:{msg,errType:err?.constructor?.name,errStr:String(err)},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'H1'})}).catch(()=>{});
      // #endregion
      toast.error(msg);
      reportSystemError({
        type: "Aggiornamento avatar fallito",
        detail: msg,
      });
    } finally {
      avatarUpdating = false;
    }
  }

  function formatMoney(amount: number | null): string {
    if (amount === null) return "—";
    return new Intl.NumberFormat("it-IT").format(amount);
  }

  function formatPlaytime(hours: number | null, minutes: number | null): string {
    if (hours === null && minutes === null) return "—";
    const h = hours ?? 0;
    const m = minutes ?? 0;
    return `${h}:${m.toString().padStart(2, "0")}`;
  }

  async function loadSavData() {
    try {
      const entries = await getSavEntries();
      // Conta giochi unici (game, non version)
      const uniqueGames = new Set(entries.map(e => e.game));
      gamesCount = uniqueGames.size;
      if (entries.length === 0) {
        lastGame = "—";
        money = null;
        playedHours = null;
        playedMinutes = null;
        return;
      }
      // Trova entry con last_played_at (mtime) più recente, altrimenti updated_at (sync)
      const latestEntry = entries.reduce((acc, e) =>
        (e.last_played_at ?? e.updated_at ?? "") > (acc.last_played_at ?? acc.updated_at ?? "") ? e : acc
      , entries[0]);
      lastGame = latestEntry?.game ?? "—";

      // Carica trainer data dal save più recente
      try {
        const trainerData = await getTrainerData(latestEntry.path);
        money = trainerData.money;
        playedHours = trainerData.playedHours;
        playedMinutes = trainerData.playedMinutes;
      } catch (err) {
        console.error("[profilo] getTrainerData failed:", err);
        money = null;
        playedHours = null;
        playedMinutes = null;
      }
    } catch {
      gamesCount = 0;
      lastGame = "—";
      money = null;
      playedHours = null;
      playedMinutes = null;
    }
  }

  async function loadPokedexCount(profileId: string) {
    try {
      const entries = await pokedexService.getPokedexState(profileId);
      const caught = entries.filter(
        (e) => (e.status ?? "").trim().toLowerCase() === "caught"
      ).length;
      pokedexCaught = caught;
    } catch (err) {
      reportSystemError({
        type: "Caricamento Pokedex dashboard fallito",
        detail: err instanceof Error ? err.message : String(err),
      });
      pokedexCaught = 0;
    }
  }

  onMount(() => {
    const profile = get(activeProfile);
    if (profile) {
      void loadPokedexCount(profile.id);
      void loadSavData();
    }
    const unsub = activeProfile.subscribe((profile) => {
      if (profile) {
        void loadPokedexCount(profile.id);
        void loadSavData();
      } else {
        pokedexCaught = 0;
        gamesCount = 0;
        lastGame = "—";
        money = null;
        playedHours = null;
        playedMinutes = null;
      }
    });
    let unlistenFile: (() => void) | undefined;
    let unlistenEntries: (() => void) | undefined;
    const refresh = () => {
      const p = get(activeProfile);
      if (p) {
        void loadPokedexCount(p.id);
        void loadSavData();
      }
    };
    listen<string>("sav-file-changed", refresh).then((fn) => (unlistenFile = fn));
    listen("sav-entries-changed", refresh).then((fn) => (unlistenEntries = fn));
    return () => {
      unsub();
      unlistenFile?.();
      unlistenEntries?.();
    };
  });
</script>

<svelte:head>
  <title>Allenatore — PokeTracker</title>
</svelte:head>

<!-- Area main: Allenatore (dashboard 2026) -->
<div
  class="flex flex-col gap-8"
  role="region"
  aria-label="Dashboard allenatore"
>
  <!-- Hero: identità in evidenza -->
  <section
    class="overflow-hidden rounded-2xl border border-[var(--border-primary)] bg-[var(--bg-tertiary)] shadow-sm transition-[box-shadow,border-color] duration-200 hover:shadow-md hover:border-[var(--border-primary)]"
    aria-labelledby="allenatore-hero-name"
  >
    <div class="flex flex-col items-center gap-6 p-8 sm:flex-row sm:items-center sm:gap-8">
      <!-- Avatar: elemento visivo principale, cliccabile per cambiare. Box nero solo per "Nessuno". -->
      <button
        type="button"
        class="group flex h-32 w-32 shrink-0 items-center justify-center overflow-hidden rounded-2xl transition-[border-color,box-shadow] duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--bg-tertiary)] {avatarId && isAvatarId(avatarId)
          ? 'hover:opacity-90'
          : 'border border-[var(--border-primary)]/60 bg-[var(--bg-primary)] hover:border-[var(--border-primary)] hover:shadow-md'}"
        onclick={() => (avatarPickerOpen = true)}
        disabled={avatarUpdating}
        aria-label="Cambia avatar"
      >
        {#if avatarId && isAvatarId(avatarId)}
          <img
            src={getAvatarSrc(avatarId)}
            alt=""
            class="h-full w-full object-contain"
          />
        {:else}
          <User
            class="text-muted-foreground/50 group-hover:text-muted-foreground/70"
            size={64}
            strokeWidth={1.25}
            aria-hidden="true"
          />
        {/if}
      </button>

      <AvatarPicker
        bind:open={avatarPickerOpen}
        selectedId={avatarId}
        onSelect={onAvatarSelect}
      />
      <!-- Nome e contesto -->
      <div class="min-w-0 flex-1 text-center sm:text-left">
        <h1
          id="allenatore-hero-name"
          class="text-2xl font-semibold tracking-tight text-foreground sm:text-3xl"
        >
          {profile?.name ?? "—"}
        </h1>
        <p class="mt-1.5 flex items-center gap-1.5 text-sm text-muted-foreground" aria-label={$watchedCount > 0 ? ($watchedCount === 1 ? "1 watcher attivo" : `${$watchedCount} watcher attivi`) : "Nessun watcher attivo"}>
          <CheckCircle class="size-4 shrink-0 {$watchedCount > 0 ? 'text-[var(--icon-success)]' : 'text-muted-foreground'}" aria-hidden="true" />
          <span>
            {$watchedCount > 0
              ? ($watchedCount === 1 ? "1 watcher attivo" : `${$watchedCount} watcher attivi`)
              : "Nessun watcher attivo"}
          </span>
        </p>
      </div>
      <!-- Blocco dati Trainer Card: 2×2 (Denaro, Tempo | Medaglie, Aggiornato) -->
      <div
        class="grid shrink-0 grid-cols-2 gap-x-14 gap-y-4 self-end sm:self-center sm:border-l sm:border-[var(--border-primary)] sm:pl-6"
        role="group"
        aria-label="Statistiche allenatore: tempo, denaro, giochi, ultimo gioco"
      >
        <!-- Riga 1: Tempo, Denaro -->
        <div class="flex flex-col items-end gap-1.5">
          <div class="flex items-center gap-1.5">
            <Clock size={18} strokeWidth={1.5} class="text-muted-foreground" aria-hidden="true" />
            <span class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">Tempo</span>
          </div>
          <p class="text-xl font-bold tabular-nums tracking-tight" class:text-muted-foreground={playedHours === null && playedMinutes === null} class:text-foreground={playedHours !== null || playedMinutes !== null}>
            {formatPlaytime(playedHours, playedMinutes)}
          </p>
        </div>
        <div class="flex flex-col items-end gap-1.5">
          <div class="flex items-center gap-1.5">
            <Coins size={18} strokeWidth={1.5} class="text-muted-foreground" aria-hidden="true" />
            <span class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">Denaro</span>
          </div>
          <p class="text-xl font-bold tabular-nums tracking-tight" class:text-muted-foreground={money === null} class:text-foreground={money !== null}>
            {money !== null ? `₽ ${formatMoney(money)}` : "—"}
          </p>
        </div>
        <!-- Riga 2: Giochi, Last -->
        <div class="flex flex-col items-end gap-1.5">
          <div class="flex items-center gap-1.5">
            <Gamepad2 size={18} strokeWidth={1.5} class="text-muted-foreground" aria-hidden="true" />
            <span class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">Giochi</span>
          </div>
          <p class="text-xl font-bold tabular-nums tracking-tight" class:text-muted-foreground={gamesCount === 0} class:text-foreground={gamesCount > 0}>
            {gamesCount}
          </p>
        </div>
        <div class="flex flex-col items-end gap-1.5">
          <div class="flex items-center gap-1.5">
            <History size={18} strokeWidth={1.5} class="text-muted-foreground" aria-hidden="true" />
            <span class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">Last</span>
          </div>
          <p class="text-xl font-bold tabular-nums tracking-tight" class:text-muted-foreground={lastGame === "—"} class:text-foreground={lastGame !== "—"}>
            {lastGame}
          </p>
        </div>
      </div>
    </div>
  </section>

  <!-- Stat tiles: Pokédex e Medaglie -->
  <section
    class="grid grid-cols-1 gap-3 sm:grid-cols-2"
    aria-label="Statistiche allenatore"
  >
    <StatCard
      label="Pokédex"
      emoji="📖"
      labelId="stat-pokedex-label"
      value={pokedexCaught}
      suffix="/{POKEDEX_GEN4_MAX}"
      suffixMuted
    />
    <StatCard
      label="Medaglie"
      emoji="🏆"
      value="—"
      muted
      labelId="stat-medaglie-label"
    />
  </section>

  <!-- Seconda card (contenuto da definire) -->
  <Card aria-labelledby="card-2-title">
    <CardHeader>
      <CardTitle id="card-2-title">Card 2</CardTitle>
    </CardHeader>
    <CardContent>
      <p class="text-sm text-muted-foreground">Contenuto da definire.</p>
    </CardContent>
  </Card>
</div>

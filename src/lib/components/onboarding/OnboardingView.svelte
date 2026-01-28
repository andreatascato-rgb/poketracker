<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { setActiveProfile } from "$lib/stores/profile";
  import type { Profile } from "$lib/stores/profile";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  let nome = $state("");
  let error = $state("");
  let submitting = $state(false);

  async function submit(e: Event) {
    e.preventDefault();
    error = "";
    const trimmed = nome.trim();
    if (!trimmed) {
      error = "Il nome non può essere vuoto.";
      return;
    }
    submitting = true;
    try {
      const created = await invoke<Profile>("create_profile", { name: trimmed });
      await setActiveProfile(created.id);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      submitting = false;
    }
  }
</script>

<div
  class="onboarding-root"
  role="main"
  aria-label="Onboarding: crea il tuo primo allenatore"
>
  <!-- Sfondo a gradiente + glow sottile (best practice 2025–2026: dark glassmorphism, depth) -->
  <div class="onboarding-bg" aria-hidden="true"></div>

  <div class="onboarding-inner">
    <header class="onboarding-brand">
      <h1 class="onboarding-app-name">PokeTracker</h1>
      <span class="onboarding-accent-line" aria-hidden="true"></span>
      <p class="onboarding-tagline">
        Crea il tuo primo allenatore per iniziare a tracciare i tuoi salvataggi.
      </p>
    </header>

    <div class="onboarding-panel">
      <form class="onboarding-form" onsubmit={submit}>
        <div class="onboarding-field">
          <Label for="onboarding-nome">Nome allenatore</Label>
          <Input
            id="onboarding-nome"
            type="text"
            bind:value={nome}
            required
            aria-required="true"
            aria-invalid={!!error}
            aria-describedby={error ? "onboarding-error" : undefined}
            placeholder="Es. Rosso"
            disabled={submitting}
            autofocus
          />
          {#if error}
            <p id="onboarding-error" class="onboarding-error" role="alert">
              {error}
            </p>
          {/if}
        </div>
        <Button
          type="submit"
          class="w-full h-11 text-base font-medium"
          disabled={submitting}
          aria-label="Crea allenatore e accedi"
        >
          {submitting ? "Creazione…" : "Inizia"}
        </Button>
      </form>
    </div>
  </div>
</div>

<style>
  .onboarding-root {
    position: relative;
    width: 100%;
    min-height: 100vh;
    min-height: 100dvh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-xl, 24px);
    box-sizing: border-box;
    color: var(--text-primary, #cccccc);
    font-family: var(--font-primary, "Segoe UI", "SF Pro Display", -apple-system, BlinkMacSystemFont, sans-serif);
    overflow: hidden;
  }

  /* Sfondo: gradiente radiale + glow morbido (tendenze 2025–2026: dark glassmorphism, depth) */
  .onboarding-bg {
    position: absolute;
    inset: 0;
    background:
      radial-gradient(ellipse 80% 50% at 50% 0%, rgba(0, 122, 204, 0.06) 0%, transparent 55%),
      radial-gradient(ellipse 100% 80% at 100% 100%, rgba(55, 55, 65, 0.5) 0%, transparent 50%),
      radial-gradient(ellipse 100% 80% at 0% 100%, rgba(40, 40, 50, 0.35) 0%, transparent 50%),
      var(--bg-primary, #1e1e1e);
    pointer-events: none;
  }

  .onboarding-inner {
    position: relative;
    width: 100%;
    max-width: 28rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2.5rem;
    text-align: center;
    animation: onboarding-in 0.5s ease-out;
  }

  @media (prefers-reduced-motion: reduce) {
    .onboarding-inner {
      animation: none;
    }
  }

  @keyframes onboarding-in {
    from {
      opacity: 0;
      transform: translateY(12px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .onboarding-brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .onboarding-app-name {
    margin: 0;
    font-size: clamp(2rem, 5.5vw, 2.75rem);
    font-weight: 600;
    letter-spacing: -0.04em;
    line-height: 1.1;
    color: var(--text-primary, #cccccc);
    background: linear-gradient(180deg, #f0f0f0 0%, #b0b0b0 100%);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .onboarding-accent-line {
    display: block;
    width: 3rem;
    height: 3px;
    border-radius: 2px;
    background: linear-gradient(
      90deg,
      rgba(0, 122, 204, 0.9) 0%,
      rgba(0, 122, 204, 0.2) 70%,
      transparent 100%
    );
  }

  .onboarding-tagline {
    margin: 0;
    font-size: 1rem;
    line-height: 1.55;
    color: var(--text-secondary, #858585);
    max-width: 28ch;
    margin-inline: auto;
    font-weight: 400;
  }

  /* Panel in stile glassmorphism (Dark Glassmorphism 2025–2026: frosted glass, blur, bordi sottili) */
  .onboarding-panel {
    width: 100%;
    padding: 2rem;
    background: rgba(37, 38, 42, 0.6);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 1rem;
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.04),
      0 4px 6px -1px rgba(0, 0, 0, 0.2),
      0 20px 40px -12px rgba(0, 0, 0, 0.4);
    text-align: left;
    animation: onboarding-panel 0.5s ease-out 0.1s both;
  }

  @media (prefers-reduced-motion: reduce) {
    .onboarding-panel {
      animation: none;
    }
  }

  @keyframes onboarding-panel {
    from {
      opacity: 0;
      transform: translateY(16px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .onboarding-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .onboarding-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .onboarding-error {
    margin: 0;
    font-size: 0.875rem;
    color: var(--destructive, #f87171);
  }
</style>

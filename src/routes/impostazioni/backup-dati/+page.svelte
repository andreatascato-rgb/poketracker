<script lang="ts">
  import { onMount } from "svelte";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import * as exportService from "$lib/services/export";
  import { reportSystemError } from "$lib/stores/error-archive";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
  } from "$lib/components/ui/card";
  import { Label } from "$lib/components/ui/label";
  import { toast } from "$lib/components/ui/sonner";
  import { FolderOpen, FolderInput, RotateCcw, Download, Upload, Archive } from "@lucide/svelte";

  let exportDirPath = $state<string>("");
  let loading = $state(true);
  let openingFolder = $state(false);
  let choosingFolder = $state(false);
  let resettingFolder = $state(false);

  async function loadExportDir() {
    try {
      const path = await exportService.getExportDir();
      exportDirPath = path ?? "";
    } catch (e) {
      reportSystemError({
        type: "Lettura cartella export fallita",
        detail: e instanceof Error ? e.message : String(e),
      });
      exportDirPath = "";
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadExportDir();
  });

  async function openExportFolder() {
    openingFolder = true;
    try {
      await exportService.openExportFolder();
    } catch (e) {
      reportSystemError({
        type: "Apertura cartella export fallita",
        detail: e instanceof Error ? e.message : String(e),
      });
    } finally {
      openingFolder = false;
    }
  }

  async function chooseFolder() {
    choosingFolder = true;
    try {
      const selected = await openDialog({
        directory: true,
        multiple: false,
      });
      if (selected) {
        await exportService.setExportDir(selected);
        exportDirPath = selected;
        toast.success("Cartella export aggiornata.");
      }
    } catch (e) {
      reportSystemError({
        type: "Impostazione cartella export fallita",
        detail: e instanceof Error ? e.message : String(e),
      });
    } finally {
      choosingFolder = false;
    }
  }

  async function useDefaultFolder() {
    resettingFolder = true;
    try {
      await exportService.setExportDir(null);
      await loadExportDir();
      toast.success("Cartella predefinita ripristinata.");
    } catch (e) {
      reportSystemError({
        type: "Ripristino cartella predefinita fallito",
        detail: e instanceof Error ? e.message : String(e),
      });
    } finally {
      resettingFolder = false;
    }
  }

  function exportPokedex() {
    toast.info("Export Pokedex in sviluppo.");
  }

  function backupNow() {
    toast.info("Backup ora in sviluppo.");
  }

  function restoreBackup() {
    toast.info("Ripristina in sviluppo.");
  }
</script>

<div class="backup-dati-page">
  <Card>
    <CardHeader>
      <CardTitle class="text-xl font-semibold min-h-9">
        Cartella export
      </CardTitle>
    </CardHeader>
    <CardContent class="flex flex-col gap-4">
      <p class="text-sm text-muted-foreground">
        Export e backup dell'app vengono salvati in questa cartella. Puoi aprirla dal file manager o sceglierne un'altra.
      </p>
      {#if loading}
        <p class="text-sm text-muted-foreground">Caricamento…</p>
      {:else}
        <div class="flex flex-col gap-2">
          <Label for="export-dir-display" class="text-xs text-muted-foreground">Percorso attuale</Label>
          <p
            id="export-dir-display"
            class="rounded-md border bg-muted/50 px-3 py-2 font-mono text-sm break-all"
            title={exportDirPath}
          >
            {exportDirPath || "—"}
          </p>
        </div>
        <div class="flex flex-wrap gap-2">
          <Button
            variant="secondary"
            onclick={openExportFolder}
            disabled={openingFolder || loading}
          >
            <FolderOpen class="size-4 shrink-0" aria-hidden="true" />
            Apri cartella export
          </Button>
          <Button
            variant="outline"
            onclick={chooseFolder}
            disabled={choosingFolder || loading}
          >
            <FolderInput class="size-4 shrink-0" aria-hidden="true" />
            Scegli cartella
          </Button>
          <Button
            variant="ghost"
            onclick={useDefaultFolder}
            disabled={resettingFolder || loading}
          >
            <RotateCcw class="size-4 shrink-0" aria-hidden="true" />
            Usa predefinita
          </Button>
        </div>
      {/if}
    </CardContent>
  </Card>

  <Card>
    <CardHeader>
      <CardTitle class="text-xl font-semibold min-h-9">
        Export e backup
      </CardTitle>
    </CardHeader>
    <CardContent class="flex flex-col gap-4">
      <p class="text-sm text-muted-foreground">
        Esporta i dati (Pokedex, backup profilo) nella cartella export, o ripristina da un backup precedente.
      </p>
      <div class="flex flex-wrap gap-2">
        <Button variant="secondary" onclick={exportPokedex}>
          <Download class="size-4 shrink-0" aria-hidden="true" />
          Export Pokedex
        </Button>
        <Button variant="secondary" onclick={backupNow}>
          <Archive class="size-4 shrink-0" aria-hidden="true" />
          Backup ora
        </Button>
        <Button variant="outline" onclick={restoreBackup}>
          <Upload class="size-4 shrink-0" aria-hidden="true" />
          Ripristina
        </Button>
      </div>
    </CardContent>
  </Card>
</div>

<style>
  .backup-dati-page {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg, 16px);
  }
</style>

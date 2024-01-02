<script lang="ts">
  import Greet from "./lib/Greet.svelte";
</script>

<main class="container">
  <div>
    <span class="label">href:</span>
    {window.location.href}
  </div>
  <div>
    <span class="label">crossOriginIsolated:</span>
    {window.crossOriginIsolated}
  </div>
  <div>
    <span class="label">hasSharedArrayBuffer:</span>
    {window.SharedArrayBuffer ? "true" : "false"}
  </div>
</main>

<style>
  .label {
    color: gray;
  }
</style>

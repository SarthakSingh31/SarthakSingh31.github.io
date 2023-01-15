<script lang="ts">
  export let id: string;
</script>

<h2 {id}>
  <slot />
</h2>

<style lang="scss">
  h2 {
    &::before {
      position: relative;
      margin-right: 10px;
      content: "0" counter(content-section) ". ";
      font-size: clamp(16px, 5vw, 28px);
      color: var(--text-secondary);
    }

    &::after {
      content: "";
      display: block;
      position: relative;
      width: 20vw;
      height: 1px;
      margin-left: 20px;
      background-color: var(--text-secondary);
    }

    display: flex;
    align-items: center;
    position: relative;

    font-family: "Raleway", sans-serif;
    font-size: clamp(22px, 5vw, 32px);
  }
</style>

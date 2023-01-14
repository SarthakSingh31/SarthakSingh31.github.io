<h2>
  <slot />
</h2>

<style lang="scss">
  h2 {
    &::before {
      position: relative;
      margin-right: 10px;
      content: "0" counter(content-section) ". ";
      font-size: 28px;
      color: var(--text-secondary);
    }

    &::after {
      content: "";
      display: block;
      position: relative;
      width: 300px;
      height: 1px;
      margin-left: 20px;
      background-color: var(--text-secondary);
    }

    display: flex;
    align-items: center;
    position: relative;

    font-family: "Raleway", sans-serif;
    font-size: 32px;
  }
</style>

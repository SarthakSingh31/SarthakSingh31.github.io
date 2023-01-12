<script lang="ts">
  export let name: string;
  export let icon: string;
  export let showcase: {
    src: string;
    alt: string;
  };
  export let showRight: boolean = false;
</script>

<li class="showcase-container" data-show-right={showRight}>
  <div class="showcase-image">
    <img src={showcase.src} alt={showcase.alt} />
  </div>
  <div class="showcase-body">
    test tset test test tset test tset test test test test test test test test
    tset test tset test test tset test tset test test test test test test test
    test tset test tset test test tset test tset test test test test test test
    test test tset test tset test test tset test tset test test test test test
    test test test tset test tset test test tset test tset test test test test
    test test test test tset test tset test test tset test tset test test test
    test test test test test tset test tset test test tset test tset test test
    test test test test test test tset test tset test test tset test tset test
    <!-- test test test test test test test tset test tset test test tset test tset
    test test test test test test test test tset test tset test test tset test
    tset test test test test test test test test tset test tset test test tset
    test tset test test test test test test test test tset test tset test test
    tset test tset test test test test test test test test tset test tset test
    test tset test tset test test test test test test test test tset test tset
    test test tset test tset test test test test test test test test tset test
    tset test test tset test tset test test test test test test test test tset
    test tset test test tset test tset test test test test test test test test
    tset test tset test test tset test tset test test test test test test test
    test tset test tset test test tset test tset test test test test test test
    test test tset test tset test test tset test tset test test test test test
    test test test tset test tset test test tset test tset test test test test
    test test test test tset test tset test test -->
  </div>
</li>

<style lang="scss">
  .showcase-container {
    list-style: none;
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));

    .showcase-image {
      grid-row: 1 / -1;

      position: relative;

      &::before {
        content: "";
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        background-color: rgba(#534582, 0.8);

        transition-property: color, background-color, border-color,
          text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 150ms;
      }

      img {
        max-width: 100%;
        height: auto;
      }
    }

    .showcase-body {
      grid-row: 1 / -1;

      z-index: 10;
    }

    &[data-show-right="false"] {
      .showcase-image {
        grid-column-start: 4;
        grid-column-end: 8;
      }
      .showcase-body {
        grid-column-start: 1;
        grid-column-end: 5;
      }
    }

    &[data-show-right="true"] {
      .showcase-image {
        grid-column-start: 1;
        grid-column-end: 5;
      }
      .showcase-body {
        grid-column-start: 4;
        grid-column-end: 8;
      }
    }

    &:hover {
      .showcase-image::before {
        background-color: rgba(purple, 0);
      }
    }
  }
</style>

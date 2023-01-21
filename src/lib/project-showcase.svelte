<script lang="ts">
  export let name: string;
  export let icon: { src: string; alt: string };
  export let showcase: {
    src: string;
    alt: string;
  };
  export let links: { src: string; alt: string; link: string }[];
  export let bottomIcons: { src: string; alt: string; link: string }[];
  export let showRight: boolean = false;

  let showModal = false;
</script>

<li class="showcase-container" data-show-right={showRight}>
  <div
    class="showcase-image"
    on:click={() => (showModal = !showModal)}
    on:keypress={() => (showModal = !showModal)}
  >
    <img src={showcase.src} alt={showcase.alt} />
  </div>
  <div class="showcase-body">
    <div class="header">
      <img src={icon.src} alt={icon.alt} class="logo-img" />
      <span>{name}</span>
    </div>
    <p>
      <slot />
    </p>
    <div class="footer">
      {#each links as link}
        <a href={link.link}>
          <img src={link.src} alt={link.alt} class="link-img" />
        </a>
      {/each}
      <span>|</span>
      {#each bottomIcons as bottomIcon}
        <a href={bottomIcon.link}>
          <img src={bottomIcon.src} alt={bottomIcon.alt} />
        </a>
      {/each}
    </div>
  </div>
  <div
    class="modal"
    data-show={showModal}
    aria-hidden={!showModal}
    on:click={() => (showModal = false)}
  >
    <div
      class="modal-content"
      on:click={(evt) => evt.stopPropagation()}
      on:keypress={(evt) => evt.stopPropagation()}
    >
      <button class="close" on:click={() => (showModal = false)}> ✕ </button>
      <img src={showcase.src} alt={showcase.alt} />
    </div>
  </div>
</li>

<style lang="scss">
  @import "../app.scss";

  .showcase-container {
    list-style: none;
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));

    margin-bottom: 32px;

    .showcase-image {
      grid-row: 1 / -1;
      display: flex;

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

        margin-top: auto;
        margin-bottom: auto;
      }
    }

    .showcase-body {
      grid-row: 1 / -1;

      z-index: 5;

      .header {
        display: flex;
        align-items: center;

        .logo-img {
          width: 62px;
        }

        span {
          font-size: 42px;
          padding-left: 16px;
          padding-right: 16px;

          font-family: "Raleway", sans-serif;
        }
      }

      p {
        // background-color: #29353d;
        background-color: var(--black);
        padding: 12px;

        border-radius: 4px;
      }

      .footer {
        display: flex;

        span {
          padding-right: 8px;
        }

        img {
          height: 24px;
          padding-right: 8px;
        }
      }
    }

    @include above-md {
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

          .header {
            flex-direction: row-reverse;
          }

          p {
            text-align: right;
          }

          .footer {
            flex-direction: row-reverse;

            span {
              padding-right: 0px;
              padding-left: 8px;
            }

            img {
              height: 24px;
              padding-right: 0px;
              padding-left: 8px;
            }
          }
        }
      }

      .showcase-image {
        cursor: pointer;

        &:hover {
          &::before {
            background-color: rgba(purple, 0);
          }
        }
      }
    }

    @include below-md {
      .showcase-image {
        grid-column-start: 1;
        grid-column-end: 8;
      }
      .showcase-body {
        grid-column-start: 1;
        grid-column-end: 8;

        padding: 16px;
      }
    }

    .modal {
      transition-property: top;
      transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
      transition-duration: 300ms;

      display: flex;
      z-index: 20;

      width: 100vw;
      height: 100vh;

      overflow-y: scroll;

      background-color: rgba(black, 0.4);

      .modal-content {
        background-color: black;

        padding: 16px;

        margin: 16px auto;
        width: 80vw;

        .close {
          background-color: unset;
          border: unset;

          color: var(--text-primary);
          font-size: 24px;

          float: right;
          padding-bottom: 8px;

          cursor: pointer;
        }

        img {
          width: 100%;
        }
      }
    }

    .modal[data-show="false"] {
      position: fixed;
      top: -100vh;
      bottom: 0;
      left: 0;
      right: 0;
    }

    .modal[data-show="true"] {
      position: fixed;
      top: 0;
      bottom: 0;
      left: 0;
      right: 0;
    }
  }
</style>

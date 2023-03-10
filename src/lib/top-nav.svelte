<script lang="ts">
  import { onMount } from "svelte";

  export let logo: { src: string; alt: string };
  export let navLinks: { text: string; link: string }[];
  export let resume: string;
  export let github: string;

  let open = false;
  let atTop = true;
  let shouldOpen = true;
  let lastScrollY = 0;

  onMount(() => {
    atTop = scrollY == 0;
    lastScrollY = scrollY;
    document.addEventListener("scroll", (evt) => {
      shouldOpen = lastScrollY > scrollY;

      atTop = scrollY == 0;
      lastScrollY = scrollY;
    });
  });
</script>

<nav data-at-top={atTop} data-should-open={shouldOpen}>
  <img src={logo.src} alt={logo.alt} class="logo" />

  <button class="open" on:click={() => (open = true)}> ☰ </button>

  <div class="links" data-open={open}>
    <button class="close" on:click={() => (open = false)}> ✕ </button>

    <ol class="list">
      {#each navLinks as navLink}
        <li><a href={navLink.link}>{navLink.text}</a></li>
      {/each}
    </ol>

    <div class="external-links">
      <a href={resume} download>Resume</a>
      <a href={github}>Github</a>
    </div>
  </div>
</nav>

<style lang="scss">
  @import "../app.scss";

  nav {
    position: fixed;
    top: 0;
    width: 100vw;
    transition-property: height top box-shadow;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 150ms;

    z-index: 10;
    background-color: rgba(var(--dark-primary), 0.85);
    height: 100px;
    backdrop-filter: blur(10px);

    &[data-at-top="false"][data-should-open="true"] {
      box-shadow: 0 10px 30px -10px rgb(43, 0, 44);
      height: 60px;
    }

    &[data-should-open="false"] {
      top: -100px;
    }

    display: flex;
    justify-content: space-between;

    .logo {
      width: 36px;
      margin-bottom: auto;
      padding: 12px;

      @include below-md {
        margin-top: auto;
      }
    }

    .list {
      li {
        &::marker {
          color: var(--text-secondary);
        }

        a {
          text-decoration: none;
          color: var(--text-primary);

          &:hover {
            color: var(--light-primary);
            transition-property: color;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 150ms;
          }
        }
      }
    }

    .external-links {
      a {
        text-decoration: none;
        color: var(--light-primary);

        padding: 8px;
        line-height: 1;
        border: 2px solid var(--light-primary);
        border-radius: 8px;

        &:hover {
          background-color: rgba(#c5b4e3, 0.3);
          // background-color: var(--light-primary-tint);
          transition-property: background-color;
          transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
          transition-duration: 150ms;
        }
      }
    }

    @include above-md {
      .open {
        display: none;
      }

      .links {
        display: flex;

        .close {
          display: none;
        }

        .list {
          display: flex;

          li {
            margin-left: 48px;
          }
        }

        .external-links {
          margin: 16px;
        }
      }
    }

    @include below-md {
      .open {
        background-color: unset;
        border: unset;

        color: var(--text-primary);

        font-size: 36px;

        margin-right: 16px;
      }

      .links {
        position: fixed;

        right: -300px;

        height: 100vh;
        background-color: var(--primary);

        transition-property: right;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 150ms;

        padding: 16px;

        &[data-open="true"] {
          right: 0px;
        }

        .close {
          // float: right;
          // margin-left: auto;
          background-color: unset;
          border: unset;

          color: var(--text-primary);
          font-size: 36px;
        }

        .list {
          // margin-top: 24px;
          // padding-left: 20px;
          text-align: center;

          font-size: 28px;
        }

        .external-links {
          font-size: 28px;
        }
      }
    }
  }
</style>

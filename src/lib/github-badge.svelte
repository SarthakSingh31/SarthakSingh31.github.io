<script lang="ts">
  import githubBook from "$lib/images/github-book.svg";
  import githubStar from "$lib/images/github-star.svg";
  import githubFork from "$lib/images/github-fork.svg";

  export let domain: { org: string; repo: string };
  export let lines: { added: number; removed: number };
  export let languageDot: { color: string; name: String };
  export let stars: string;
  export let forks: string;

  $: numGreen = Math.floor((5 * lines.added) / (lines.added + lines.removed));
  $: numRed = Math.floor((5 * lines.removed) / (lines.added + lines.removed));
  $: numMuted = 5 - numGreen - numRed;
</script>

<li class="container">
  <div class="header">
    <div class="link">
      <img src={githubBook} alt={domain.repo} aria-hidden="true" />
      <a href="https://github.com/{domain.org}/{domain.repo}">
        <span>{domain.org}</span><span class="bold">/{domain.repo}</span>
      </a>
    </div>
    <div class="change">
      <span class="added">+{lines.added}</span>
      <span class="removed">-{lines.removed}</span>
      {#each Array(numGreen) as _}<span
          class="green"
        />{/each}{#each Array(numRed) as _}<span
          class="red"
        />{/each}{#each Array(numMuted) as _}<span class="muted" />{/each}
    </div>
  </div>
  <div class="content">
    <slot />
  </div>
  <div class="footer">
    <span>
      <span class="dot" style="background-color: {languageDot.color}" />
      {languageDot.name}
    </span>
    <span><img src={githubStar} alt="stars" /> {stars}</span>
    <span><img src={githubFork} alt="forks" /> {forks}</span>
  </div>
</li>

<style lang="scss">
  .container {
    list-style: none;
    background-color: var(--black);
    padding: 16px;

    margin-bottom: 8px;

    img {
      filter: invert(60%);
      vertical-align: middle;

      height: 18px;
      width: 18px;
    }

    .header {
      display: flex;
      justify-content: space-between;

      .link {
        margin-bottom: 8px;
        a {
          text-decoration: none;
          color: #58a6ff;

          .bold {
            font-weight: 600;
          }

          &:hover {
            text-decoration: underline;
          }
        }
      }

      .change {
        .added {
          color: var(--green);
        }
        .removed {
          color: var(--red);
        }

        .green,
        .red,
        .muted {
          content: "";
          width: 12px;
          height: 12px;
          display: inline-block;

          outline: 1px solid rgba(black, 0.1);
          margin: 1px;

          vertical-align: middle;
        }

        .green {
          background-color: var(--green);
        }

        .red {
          background-color: var(--red);
        }

        .muted {
          background-color: var(--muted);
        }
      }
    }

    .content {
      margin-bottom: 8px;
    }

    .content,
    .footer {
      font-size: 14px;
      color: var(--text-slate);
    }

    .dot {
      width: 16px;
      height: 16px;
      display: inline-block;
      border-radius: 8px;

      vertical-align: sub;
    }
  }
</style>

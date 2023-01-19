<script lang="ts">
  import GithubBadge from "./github-badge.svelte";
</script>

<div>
  <ol>
    <GithubBadge
      domain={{ org: "rust-lang", repo: "rust" }}
      lines={{ added: 129, removed: 100 }}
      languageDot={{ color: "#dea584", name: "Rust" }}
      stars="76.7k"
      forks="10.2k"
    >
      Empowering everyone to build reliable and efficient software.
    </GithubBadge>
    <GithubBadge
      domain={{ org: "bevyengine", repo: "bevy" }}
      lines={{ added: 59, removed: 47 }}
      languageDot={{ color: "#dea584", name: "Rust" }}
      stars="21.4k"
      forks="2.1k"
    >
      A refreshingly simple data-driven game engine built in Rust
    </GithubBadge>
    <GithubBadge
      domain={{ org: "starship", repo: "starship" }}
      lines={{ added: 4, removed: 3 }}
      languageDot={{ color: "#dea584", name: "Rust" }}
      stars="32k"
      forks="1.4k"
    >
      The minimal, blazing-fast, and infinitely customizable prompt for any
      shell!
    </GithubBadge>
    <GithubBadge
      domain={{ org: "SarthakSingh31", repo: "bevy_4x_camera" }}
      lines={{ added: 5397, removed: 4774 }}
      languageDot={{ color: "#dea584", name: "Rust" }}
      stars="30"
      forks="10"
    >
      A 4X style camera for bevy
    </GithubBadge>
    <GithubBadge
      domain={{ org: "SarthakSingh31", repo: "bevy_tile_map" }}
      lines={{ added: 3461, removed: 942 }}
      languageDot={{ color: "#dea584", name: "Rust" }}
      stars="3"
      forks="1"
    >
      A tile map rendering plugin for bevy
    </GithubBadge>
    <GithubBadge
      domain={{ org: "uw-it-aca", repo: "myuw-android" }}
      lines={{ added: 6366, removed: 3061 }}
      languageDot={{ color: "#A97BFF", name: "Kotlin" }}
      stars="0"
      forks="0"
    >
      An android wrapper for the MyUW website
    </GithubBadge>
  </ol>
</div>

<style lang="scss">
  ol {
    padding-left: 0;
  }
</style>

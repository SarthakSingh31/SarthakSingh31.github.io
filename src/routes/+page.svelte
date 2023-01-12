<script lang="ts">
  import androidLogo from "$lib/images/android-logo.ico";
  import myuw_icon from "$lib/images/myuw-icon.png";
  import myuw_showcase from "$lib/images/myuw-showcase.png";

  import ProjectShowcase from "../lib/project-showcase.svelte";
  import TopNav from "$lib/top-nav.svelte";
  import Intro from "$lib/intro.svelte";
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
  <link
    href="https://fonts.googleapis.com/css2?family=Raleway:wght@700&family=Roboto+Mono&display=swap"
    rel="stylesheet"
  />
</svelte:head>

<div class="outer-container">
  <TopNav
    logo={{ src: androidLogo, alt: "Test" }}
    navLinks={[
      { link: "#about", text: "About Me" },
      { link: "#showcase", text: "Showcase" },
      { link: "#open-source", text: "Open Source" },
      { link: "#contact-me", text: "Contact Me" },
    ]}
    resume="SarthakSinghResume_nov_2021.pdf"
    github="https://github.com/SarthakSingh31/"
  />

  <div class="main-container">
    <section>
      <Intro />
    </section>

    <section>
      02. Some Things I've Built

      <ProjectShowcase
        name="MyUW"
        icon={myuw_icon}
        showcase={{ src: myuw_showcase, alt: "MyUW home page" }}
      />

      <ProjectShowcase
        name="test"
        icon=""
        showcase={{ src: myuw_showcase, alt: "Test" }}
        showRight
      />

      <ProjectShowcase
        name="test"
        icon=""
        showcase={{ src: myuw_showcase, alt: "Test" }}
      />
    </section>
  </div>
</div>

<style lang="scss">
  :global(:root) {
    --sm-break-point: 576px;
    --md-break-point: 768px;
    --lg-break-point: 992px;
    --xl-break-point: 1200px;
    --2xl-break-point: 1400px;

    --primary: #534582;
    --dark-primary: #221c35;
    --light-primary: #c5b4e3;
    --light-primary-tint: rgba(#c5b4e3, 0.3);

    --text-primary: white;
    --text-secondary: #c5b4e3;
    --text-slate: #8892b0;
  }

  :global(body) {
    margin: 0;

    font-family: "Roboto Mono", monospace;
  }

  .main-container {
    padding: 0 50px;
  }

  .outer-container {
    background-color: var(--dark-primary);
    color: var(--text-primary);

    min-height: 100vh;
  }

  section {
    max-width: 1000px;
    margin: auto;
  }
</style>

<script lang="ts">
  import androidLogo from "$lib/images/android-logo.ico";
  import myuw_icon from "$lib/images/myuw-icon.png";
  import myuw_showcase from "$lib/images/myuw-showcase.png";

  import ProjectShowcase from "$lib/project-showcase.svelte";
  import TopNav from "$lib/top-nav.svelte";
  import Intro from "$lib/intro.svelte";
  import SectionHeading from "$lib/section-heading.svelte";
  import AboutMe from "$lib/about-me.svelte";
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

    <section class="content">
      <SectionHeading id="about">About Me</SectionHeading>
      <AboutMe />
    </section>

    <section class="content">
      <SectionHeading id="showcase">Some Things I've Built</SectionHeading>

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
    <section class="content">
      <SectionHeading id="open-source">
        My Open Source Contributions
      </SectionHeading>
    </section>
    <section class="content">
      <SectionHeading id="Contact Me">Contact Me</SectionHeading>
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
  }

  section {
    max-width: 1000px;
    margin: auto;
  }

  .content {
    counter-increment: content-section 1;

    display: flex;
    // justify-content: center;
    flex-direction: column;
    min-height: 100vh;

    // margin: auto;
  }
</style>

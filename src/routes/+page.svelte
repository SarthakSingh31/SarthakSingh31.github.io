<script lang="ts">
  import android_logo from "$lib/images/android-logo.ico";
  import myuw_icon from "$lib/images/myuw.png";
  import myuw_showcase from "$lib/images/myuw-showcase.png";
  import vue_icon from "$lib/images/vue-logo.svg";
  import play_store from "$lib/images/play-store-logo.png";
  import github_logo_light from "$lib/images/github-logo-light.png";
  import gcp_logo from "$lib/images/gcp-logo.svg";
  import javascript_logo from "$lib/images/javascript.svg";
  import bootstrap_logo from "$lib/images/bootstrap-logo.png";
  import sass_logo from "$lib/images/sass.png";
  import django_logo from "$lib/images/django-logo.png";
  import spacescout_logo from "$lib/images/scout.png";
  import scout_showcase from "$lib/images/scout-showcase.png";
  import foodalert_logo from "$lib/images/foodalert.png";
  import foodalert_showcase from "$lib/images/foodalert-showcase.png";

  import ProjectShowcase from "$lib/project-showcase.svelte";
  import TopNav from "$lib/top-nav.svelte";
  import Intro from "$lib/intro.svelte";
  import SectionHeading from "$lib/section-heading.svelte";
  import AboutMe from "$lib/about-me.svelte";
  import ContactMe from "$lib/contact-me.svelte";
  import OpenSource from "$lib/open-source.svelte";
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
  <link
    href="https://fonts.googleapis.com/css2?family=Raleway:wght@700&family=Roboto+Mono:wght@400;600&display=swap"
    rel="stylesheet"
  />

  <title>Sarthak Singh</title>
</svelte:head>

<div class="outer-container">
  <TopNav
    logo={{ src: "favicon.png", alt: "Test" }}
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
        icon={{ src: myuw_icon, alt: "MyUW" }}
        showcase={{ src: myuw_showcase, alt: "MyUW home page" }}
        links={[
          {
            src: github_logo_light,
            alt: "Github",
            link: "https://github.com/uw-it-aca/myuw",
          },
          {
            src: play_store,
            alt: "Play Store",
            link: "https://play.google.com/store/apps/details?id=edu.uw.myuw_android",
          },
        ]}
        bottomIcons={[
          {
            src: django_logo,
            alt: "Django",
            link: "https://www.djangoproject.com/",
          },
          {
            src: javascript_logo,
            alt: "JavaScript",
            link: "https://www.javascript.com/",
          },
          { src: vue_icon, alt: "Vue.js", link: "https://vuejs.org/" },
          {
            src: bootstrap_logo,
            alt: "Bootstrap",
            link: "https://getbootstrap.com/",
          },
          {
            src: sass_logo,
            alt: "Sass Lang",
            link: "https://sass-lang.com/",
          },
          {
            src: gcp_logo,
            alt: "Google Cloud Platform",
            link: "https://cloud.google.com/",
          },
        ]}
      >
        The main student portal for the University of Washington. Provides the
        students with a visual academic calendar, the latest notifcations, and
        quick links to important pages.
      </ProjectShowcase>

      <ProjectShowcase
        name="Scout"
        icon={{ src: spacescout_logo, alt: "Scout" }}
        showcase={{ src: scout_showcase, alt: "Scout" }}
        links={[
          {
            src: github_logo_light,
            alt: "Github",
            link: "https://github.com/uw-it-aca/scout-android",
          },
          {
            src: play_store,
            alt: "Play Store",
            link: "https://play.google.com/store/apps/details?id=edu.uw.UWScout",
          },
        ]}
        bottomIcons={[
          {
            src: django_logo,
            alt: "Django",
            link: "https://www.djangoproject.com/",
          },
          {
            src: javascript_logo,
            alt: "JavaScript",
            link: "https://www.javascript.com/",
          },
          {
            src: android_logo,
            alt: "Android",
            link: "https://www.android.com/",
          },
          {
            src: sass_logo,
            alt: "Sass Lang",
            link: "https://sass-lang.com/",
          },
        ]}
        showRight
      >
        Provides students with an easily filterable list of places to study at
        and borrow tech equipment from.
      </ProjectShowcase>

      <ProjectShowcase
        name="FoodAlert"
        icon={{ src: foodalert_logo, alt: "FoodAlert" }}
        showcase={{ src: foodalert_showcase, alt: "FoodAlert" }}
        links={[
          {
            src: github_logo_light,
            alt: "Github",
            link: "https://github.com/uw-it-aca/uw-foodalert/tree/develop",
          },
        ]}
        bottomIcons={[
          {
            src: javascript_logo,
            alt: "JavaScript",
            link: "https://www.javascript.com/",
          },
          { src: vue_icon, alt: "Vue.js", link: "https://vuejs.org/" },
          {
            src: bootstrap_logo,
            alt: "Bootstrap",
            link: "https://getbootstrap.com/",
          },
          {
            src: sass_logo,
            alt: "Sass Lang",
            link: "https://sass-lang.com/",
          },
          {
            src: gcp_logo,
            alt: "Google Cloud Platform",
            link: "https://cloud.google.com/",
          },
        ]}
      >
        Notifies the user of any food leftover from events organized by the
        university.
      </ProjectShowcase>
    </section>
    <section class="content">
      <SectionHeading id="open-source">
        My Open Source Contributions
      </SectionHeading>
      <OpenSource />
    </section>
    <section class="content">
      <SectionHeading id="contact-me">Contact Me</SectionHeading>
      <ContactMe />
    </section>
  </div>
</div>

<style lang="scss">
  @import "../app.scss";

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

    --red: #f85149;
    --green: #3fb950;
    --muted: rgba(110, 118, 129, 0.4);
    --black: #0d1117;

    --text-primary: white;
    --text-secondary: #c5b4e3;
    --text-slate: #8892b0;
  }

  :global(body) {
    margin: 0;

    font-family: "Roboto Mono", monospace;
  }

  .main-container {
    @include above-sm {
      padding: 0 50px;
    }
    @include below-sm {
      padding: 0 20px;
    }
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
    min-height: 90vh;

    // margin: auto;
  }
</style>

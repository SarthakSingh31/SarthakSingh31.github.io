<script lang="ts">
  import myImage from "$lib/images/me.png";
</script>

<div class="container">
  <div class="content">
    <p>
      Hello! My name is Sarthak and I enjoy creating things that live on the
      internet. My interest in web development started back in 2014 when I
      decided to create a webpage for my school's tech department — turns out
      hacking together a custom fourm taught me a lot about HTML & CSS!
    </p>
    <p>
      Fast-forward to today, and I’ve had the privilege of working at UW AXDD, a
      team within University of Washington dedicated towards building
      applications to aid students in achiving their most at the university.
    </p>
    <p>
      While working here I've had the change of build everything from mobile
      apps to websites.
    </p>
    <p>Here are a few technologies I’ve been working with recently:</p>
    <ol>
      <li>JavaScript</li>
      <li>TypeScript</li>
      <li>Vue.js</li>
      <li>Node.js</li>
      <li>SCSS</li>
      <li>Kotlin</li>
    </ol>
  </div>
  <img src={myImage} alt="Sarthak Singh" class="image" />
</div>

<style lang="scss">
  @import "../app.scss";

  .container {
    display: flex;

    @include below-sm {
      flex-direction: column;
    }

    .content {
      @include above-sm {
        margin-right: 32px;
      }
      text-align: justify;
    }

    .image {
      box-shadow: 16px 16px white;

      @include above-sm {
        margin-bottom: auto;
        height: 30vw;
        width: 30vw;
        max-height: 400px;
        max-width: 400px;
      }

      @include below-sm {
        margin: 16px;
      }
    }
  }
</style>

<div class="wrapper">
  <h1>Hi, my name is</h1>
  <h2>Sarthak Singh.</h2>
  <h3>I build things for the web.</h3>
  <p>
    I’m a software engineer specializing in building (and occasionally
    designing) exceptional digital experiences. Currently, I'm focused on
    completing my bachelor's and contributing to open source software.
  </p>
  <div>
    <a href="https://github.com/SarthakSingh31/">Checkout my Github!</a>
  </div>
</div>

<style lang="scss">
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    height: 100vh;

    margin: auto;
  }

  h1 {
    color: var(--text-secondary);
    font-size: clamp(14px, 5vw, 16px);
  }

  h2,
  h3 {
    font-family: "Raleway", sans-serif;
    font-weight: 700;
    font-size: clamp(40px, 8vw, 80px);
    margin: 0;
  }

  p,
  h3 {
    color: var(--text-slate);
  }

  p {
    max-width: 540px;
  }

  div {
    margin-top: 48px;

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
</style>

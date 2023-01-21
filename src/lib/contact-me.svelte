<script lang="ts">
  let showToast = false;
  let toastMessage: string | null = null;

  function postForm(evt: Event) {
    evt.preventDefault();

    let name = (<HTMLInputElement>document.querySelector('input[name="name"]'))
      .value;
    let email = (<HTMLInputElement>(
      document.querySelector('input[name="email"]')
    )).value;
    let subject = (<HTMLInputElement>(
      document.querySelector('input[name="subject"]')
    )).value;
    let message = (<HTMLInputElement>(
      document.querySelector('textarea[name="message"]')
    )).value;

    fetch(
      "http://34.123.210.24:80/api/v1/form?name=" +
        name +
        "&email=" +
        email +
        "&subject=" +
        subject +
        "&message=" +
        message,
      {
        method: "PUT",
      }
    )
      .then((resp) => {
        console.log(resp);
        toastMessage = "Message Sent Successfully!";
        showToast = true;
      })
      .catch((err) => {
        console.error(err);
        toastMessage =
          "Failed to send the message. Please send it through email instead.";
        showToast = true;
      });
  }
</script>

<p>
  I’m interested in freelance opportunities – regardless of the projects size.
  If you have a request or question, don’t hesitate to use this form or email me
  at
  <a href="mailto:sarthak.singh99@gmail.com">sarthak.singh99@gmail.com</a>.
</p>

<div>
  <form on:submit={postForm}>
    <div class="name-email">
      <input name="name" placeholder="Name" required />
      <input name="email" type="email" placeholder="Email" required />
    </div>
    <div class="subject">
      <input name="subject" placeholder="Subject" required />
    </div>
    <div class="message">
      <textarea name="message" placeholder="Message" required />
    </div>
    <div class="submit">
      <input type="submit" value="Submit" />
    </div>
  </form>
  <div class="toast" data-show={showToast}>
    <div class="toast-body">
      <button class="close" on:click={() => (showToast = false)}> ✕ </button>
      {toastMessage}
    </div>
  </div>
</div>

<style lang="scss">
  a {
    color: var(--text-secondary);
  }

  form {
    input[required],
    textarea {
      height: clamp(16px, 5vw, 36px);

      font-size: clamp(12px, 3vw, 24px);
      padding: 8px;

      margin-bottom: 16px;
    }

    .name-email {
      display: flex;
      margin-top: 32px;
      input {
        width: 100%;

        &:nth-child(1) {
          margin-right: 16px;
        }
      }
    }

    .subject {
      display: flex;
      input {
        width: 100%;
      }
    }

    .message {
      display: flex;
      textarea {
        width: 100%;
        height: 144px;
      }
    }

    .submit {
      display: flex;

      input[type="submit"] {
        margin: auto;

        color: var(--light-primary);
        font-size: 32px;

        margin-top: 32px;
        margin-bottom: 32px;
        padding: 16px;
        line-height: 1;
        background: none;
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
  }

  .toast {
    transition-property: top;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 300ms;

    .toast-body {
      background-color: black;
      z-index: 20;

      float: right;

      padding: 16px;
      border-radius: 4px;

      margin: 16px;
      max-width: 300px;

      .close {
        background-color: unset;
        border: unset;

        color: var(--text-primary);
        // font-size: 24px;

        float: right;
        margin-left: 8px;

        cursor: pointer;
      }
    }

    height: 100vh;
  }

  .toast[data-show="false"] {
    position: fixed;
    top: 100vh;
    bottom: 0;
    left: 0;
    right: 0;

    // display: none;
  }

  .toast[data-show="true"] {
    position: fixed;
    top: 80vh;
    bottom: 0;
    left: 0;
    right: 0;
  }
</style>

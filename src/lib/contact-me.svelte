<p>
  I’m interested in freelance opportunities – regardless of the projects size.
  If you have a request or question, don’t hesitate to use this form or email me
  at
  <a href="mailto:sarthak.singh99@gmail.com">sarthak.singh99@gmail.com</a>.
</p>

<form>
  <div class="name-email">
    <input placeholder="Name" required />
    <input type="email" placeholder="Email" required />
  </div>
  <div class="subject">
    <input placeholder="Subject" required />
  </div>
  <div class="message">
    <textarea placeholder="Message" required />
  </div>
  <div class="submit">
    <input type="submit" value="Submit" />
  </div>
</form>

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
</style>

import React from 'react';

export class Footer extends React.Component {
  render() {
    return (
      <div id="footer" className="row">
        <a href="http://www.github.com/dgibb/moes">
          <img src="/img/GitHub.png" height="30px" alt="GitHub" />
        </a>
        <a href="mailto:dgibb802@gmail.com">
          <img src="/img/Gmail.png" height="30px" alt="Gmail" />
        </a>
        <a href="https://www.linkedin.com/in/david-gibb-a04816b6/">
          <img src="/img/LinkedIn.png" height="30px" alt="LinkedIn" />
        </a>
      </div>
    );
  }
}

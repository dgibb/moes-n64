import React from 'react';
import jquery from 'jquery';
import { Link } from 'react-router-dom';

export class Header extends React.Component {

sendResetRequest(){
  console.log('sending reset request');
  jquery.post('/reset');
}

  render() {
    return (
      <div id="header-container"className="container-fluid row">
        <div className="col-lg-1 col-md-1 col-sm-0 col-xs-0" />
        <div id="header" className="col-lg-10 col-md-10 col-sm-12 col-xs-12">
          <div id="name">
            MOES:N64
          </div>
        </div>
      </div>
    );
  }
}

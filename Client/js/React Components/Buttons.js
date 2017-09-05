import React from 'react';
import PropTypes from 'prop-types';

export class Buttons extends React.Component {
  sendRom() {
    const input = document.getElementById('romFileInput');
    const file = input.files[0];
    if (file === undefined) {
      alert('Please Select a File');
      return;
    }
    const reader = new FileReader;
    reader.onload = (e) => {
      const byteArray = new Uint8Array(reader.result);
      console.log(byteArray);
      xhr.send(byteArray);
    };
    const xhr = new XMLHttpRequest;
    xhr.open('POST', '/sendRom', false);
    reader.readAsArrayBuffer(file);
  }


  render() {
    if (this.props.romLoaded) {
      return (
        <div className="row">
          <div className="col-lg-3 col-md-2 col-sm-1 col-xs-1" />
          <div className="col-lg-6 col-md-8 col-sm-10 col-xs-10" >
            <div className="row">
              <h1>Loaded</h1>
            </div>
          </div>
        </div>
      );
    }
    return (
      <div className="outer-container white row">
        <div className="inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
          <h2 className="large-margin">Choose a ROM File</h2>
          <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
          <div className="col-lg-3 col-md-3 col-sm-3 col-xs-3" />
            <input type="file" id="romFileInput" name="file[]" />
          </div>
          <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
            <button id="load-ROM" onClick={this.sendRom}>Play ROM</button>
          </div>
        </div>
      </div>
    );
  }
}

Buttons.propTypes = {
  romLoaded: PropTypes.bool.isRequired,
};

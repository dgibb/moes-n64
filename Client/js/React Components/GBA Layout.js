import React from 'react';
import jquery from 'jquery'
import { Header } from './Header';
import { CpuState } from './CpuState';
import { Footer } from './Footer';

export class Layout extends React.Component {
  constructor() {
    super();
    this.state = {
      romLoaded: false,
      mode: 'User',
      thumb: 'ARM',
      registers: [
        875, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 'N/A', 0,
      ],
      buttonText: 'Play ROM',
      buttonFunc: this.sendRom.bind(this),
    };
    this.sendRom = this.sendRom.bind(this);
    this.resetServer = this.resetServer.bind(this);
    this.executeInstruction = this.executeInstruction.bind(this);
  }

  executeInstruction() {
    jquery.get('/runBlock', () => {
      // const cpuState = JSON.parse(data)
      // this.setState({
      //  registers: cpuState.R,
      //  thumb: cpuState.Thumb,
      //  mode: cpuState.Mode
      // })
    });
  }

  resetServer() {
    jquery.get('/regIncrease', (data) => {
      const cpuState = JSON.parse(data);
    });
  }

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
    this.setState({
      romLoaded: true,
      buttonFunc: this.executeInstruction.bind(this),
      buttonText: 'Run Instruction',
    });
  }


  render() {
    if (!this.state.romLoaded) {
      return (
        <div>
          <Header />

          <div id="middle-content" className="row" >
            <div className="page-title-container outer-container row home">
              <div className="col-lg-2 col-md-1 col-sm-0 col-xs-0" />
              <div className="title-inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
                <div className="title-container">
                  <h1>MIPS to x86 Dynamic Recompiler</h1>
                </div>
              </div>
            </div>

            <CpuState
              registers={this.state.registers}
              mode={this.state.mode}
              thumb={this.state.thumb}
              romLoaded={this.state.romLoaded}
            />
            <div className="outer-container white row">
              <div className="inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
                <h2 className="large-margin">Choose a ROM File</h2>
                <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
                  <div className="col-lg-3 col-md-3 col-sm-3 col-xs-3" />
                  <input type="file" id="romFileInput" name="file[]" />
                </div>
                <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
                  <button id="load-ROM" onClick={this.state.buttonFunc}>{this.state.buttonText}</button>
                </div>
              </div>
            </div>
          </div>
          <Footer />
        </div>
      );
    }

    return (
      <div>
        <Header />

        <div id="middle-content" className="row" >
          <div className="page-title-container outer-container row home">
            <div className="col-lg-2 col-md-1 col-sm-0 col-xs-0" />
            <div className="title-inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
              <div className="title-container">
                <h1>MIPS to x86 Dynamic Recompile</h1>
              </div>
            </div>
          </div>

          <CpuState
            registers={this.state.registers}
            mode={this.state.mode}
            thumb={this.state.thumb}
            romLoaded={this.state.romLoaded}
          />
          <div className="outer-container white row">
            <div className="inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
              <h2 className="large-margin">Cpu Controls</h2>
              <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
                <div className="col-lg-3 col-md-3 col-sm-3 col-xs-3" />
                <button id="load-ROM" onClick={this.resetServer}>Reset</button>
              </div>
              <div className="button-container col-lg-6 col-md-6 col-sm-6 col-xs-6">
                <button id="load-ROM" onClick={this.state.buttonFunc}>{this.state.buttonText}</button>
              </div>
            </div>
          </div>
        </div>
        <Footer />
      </div>
    );
  }
}

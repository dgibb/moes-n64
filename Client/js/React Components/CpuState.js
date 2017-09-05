import React from 'react';
import PropTypes from 'prop-types';
import { Register } from './Register';

export class CpuState extends React.Component {

  toHex(value) {
    let val = value.toString(16);
    //console.log(val)
    while (val.length < 8) {
      val = '0' + val;
      //consol.log (val.length)
    }
    return (val.toUpperCase());
  }

  disabled(i) {
    switch (i) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
      case 6:
      case 7:
      case 15:
      case 16:
      case 18:
        return false;

      case 8:
      case 9:
      case 10:
      case 11:
      case 12:
      case 13:
      case 14:
        if (this.props.thumb === 'Thumb') {
          return true;
        }
        return false;

      case 17:
        if (this.props.mode === 'User') {
          return true;
        }
        return false;

      default:
        return true;
    }
  }

  render() {
    if (!this.props.romLoaded) {
      return (
        <div className="outer-container row grey">
          <div className="row">
            <div className="inner-container col-lg-8 col-md-10 col-sm-12 col-xs-12">
              <h2>About MOES:N64</h2>
              <p>MOES Nintendo64 Emulator is currently in its early stages. Until more progress is made you can attempt to execute ROM files here on the MIPS to x86 dynamic recompiler being used as the emulators core. Please send me an email if you find any bugs or need to get in contact for any reason.</p>
            </div>
          </div>
        </div>
      );
    }

    const registerNames = [
      'r0', 'r1', 'r2', 'r3', 'r4', 'r5', 'r6', 'r7',
      'r8', 'r9', 'r10', 'r11', 'r12', 'r13 (SP)',
      'r14 (LR)', 'r15 (PC)', 'CPSR', 'SPSR', 'IF/ID register'
    ];
    const registers = this.props.registers.map(
      (register, i) => (
        <Register
          key={i}
          leftText={registerNames[i]}
          rightText={(register === 'N/A') ? 'N/A' : '0x' + this.toHex(register)}
          disabled={this.disabled(i)}
        />
      ));

    return (
      <div className=" outer-container grey ">
        <div className="row" >
          <div id="register-box" className=" inner-container col-lg-6 col-md-8 col-sm-10 col-xs-10 bottom-border-1" >

            <div className="row bottom-border-2">
              <h1 className="register">Registers</h1>
            </div>

            <div className="row no-pad bottom-border-2">

              <div className="col-lg-6 col-md-6 col-sm-6 col-xs-6 left-column" >
                <h2 className="register"><strong>{this.props.mode} Mode</strong></h2>
              </div>

              <div className="col-lg-6 col-md-6 col-sm-6 col-xs-6" >
                <h2 className="register"><strong>{this.props.thumb} Mode</strong></h2>
              </div>

            </div>

            <div className="row no-pad">
              {registers}
            </div>

          </div>
        </div>
      </div>
    );
  }
}

CpuState.propTypes = {
  mode: PropTypes.string.isRequired,
  thumb: PropTypes.string.isRequired,
  registers: PropTypes.array.isRequired,
  romLoaded: PropTypes.bool.isRequired,
};

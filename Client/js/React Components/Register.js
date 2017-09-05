import React from 'react';
import PropTypes from 'prop-types';

export class Register extends React.Component {

  render() {

    let regClass = 'register'
    regClass += (this.props.disabled) ? ' disabled' : '';

    return (
      <div className="col-lg-12 col-md-12 col-sm-12 col-xs-12 no-gutters">
        <div className="col-lg-2 col-md-2 col-sm-2 col-xs-2 left-column bottom-border-1">
          <p className="register">{this.props.leftText}</p>
        </div>
        <div className="col-lg-10 col-md-10 col-sm-10 col-xs-10 bottom-border-1">
          <p className={regClass}>{this.props.rightText}</p>
        </div>
      </div>
    );
  }
}

Register.propTypes = {
  leftText: PropTypes.string.isRequired,
  rightText: PropTypes.string.isRequired,
  disabled: PropTypes.bool.isRequired,
};

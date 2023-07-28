import React, { Fragment } from 'react';

// This is a component. A component is a reusable piece of code that represents
// part of a user interface.
// Components render, manage and update UI elements in our application.

// Define a function called square, export makes it visible outside this file.
// Default means that this is effectivley the main() of this file.
export default function Board() {
  // <button> is a JSX Element.
  // A JSX Element is a combination of Javascript, and HTML tags which defines
  // what we want to display.
  // It's slightly different to normal CSS. Case in point className is what
  // defines the css class as class is a js keyword.

  // Components must return a single JSX expression. To solve this we often wrap
  // our code inside "fragment" tags
  return (
    <Fragment>
      <div className="board-row">
        <Square value="1" />
        <Square value="2" />
        <Square value="3" />
      </div>
      <div className="board-row">
        <Square value="4" />
        <Square value="5" />
        <Square value="6" />
      </div>
      <div className="board-row">
        <Square value="7" />
        <Square value="8" />
        <Square value="9" />
      </div>
    </Fragment>
  );
}

// Let's make our boards square a re-usable component.
// Our square has some state, so we will handle that with Properties.
// Our square function takes a property from the board, by the name of value
function Square({ value }) {
  // We can store a closure inside another function, and use that as our
  // onclick function!
  function handleClick() {
    console.log("Clicked space ", value);
  }

  return (
    <button className="square" onClick={handleClick}>{value}</button>
  )
}

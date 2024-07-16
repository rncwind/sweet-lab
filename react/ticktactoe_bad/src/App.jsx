import React, { Fragment, useState } from 'react';

export default function Board() {
  // We move our state into here to make our Square component stateless.
  const [squares, setSquares] = useState(Array(9).fill(null));
  // Handle changing the turns
  const [xIsNext, setXIsNext] = useState(true);

  // See if we have a winner.
  const winner = calculateWinner(squares);
  // Creeate a vairable to hold the status of the game.
  let status = null;
  if (winner) {
    status = "Winner: " + winner;
  } else {
    status = "Next Player: " + (xIsNext ? "X" : "O");
  }

  // Since board now manages our state, we need to create a way for Board to
  // interact with Sqaure
  // To do this, we make a closure here, that we pass down into Square.

  function handleClick(i) {
    // Clone the squraes array.
    const nextSquares = squares.slice();
    // Check if the value already in the array is "truthy"
    // this prevents being able to overwrite other players!
    if (squares[i]) {
      return;
    }
    // Check if we have a winner!
    if (winner) {
      return;
    }
    // Check which players turn it is
    if (xIsNext) {
      nextSquares[i] = 'X';
    } else {
      nextSquares[i] = 'O';
    }
    // Set squares to our new copy.
    setSquares(nextSquares);
    // Change turn
    setXIsNext(!xIsNext);
  }


  return (
    <Fragment>
      <div className="status">{status}</div>
      <div className="board-row">
        <Square value={squares[0]} onSquareClick={() => handleClick(0)} />
        <Square value={squares[1]} onSquareClick={() => handleClick(1)} />
        <Square value={squares[2]} onSquareClick={() => handleClick(2)} />
      </div>
      <div className="board-row">
        <Square value={squares[3]} onSquareClick={() => handleClick(3)} />
        <Square value={squares[4]} onSquareClick={() => handleClick(4)} />
        <Square value={squares[5]} onSquareClick={() => handleClick(5)} />
      </div>
      <div className="board-row">
        <Square value={squares[6]} onSquareClick={() => handleClick(6)} />
        <Square value={squares[7]} onSquareClick={() => handleClick(7)} />
        <Square value={squares[8]} onSquareClick={() => handleClick(8)} />
      </div>
    </Fragment>
  );
}

// Value is either 'O', 'X' or null
// onSquareClick is the closure passed from Board that defines how we mutate
// the state.
function Square({ value, onSquareClick }) {
  return (
    <button className="square" onClick={onSquareClick}>{value}</button>
  )
}

function calculateWinner(squares) {
  const lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
  ]
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return squares[a];
    }
  }
}

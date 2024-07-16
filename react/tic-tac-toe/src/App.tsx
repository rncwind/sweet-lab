import { useState } from 'react'
import './App.css'

// Given a value for the square (X or O) and a closure to mutate the square's state
// create a Square component.
function Square({ value, onSquareClick }) {
  return (
    <>
      <button className="square" onClick={onSquareClick}>{value}</button>
    </>
  )
}

function Game() {
  // Lift some state up into here.
  //const [xIsNext, setXIsNext] = useState(true);
  // History array, store previous game states.
  const [history, setHistory] = useState([Array(9).fill(null)]);
  // Current move id of the board.
  const [currentMove, setCurrentMove] = useState(0);
  const xIsNext = currentMove % 2 === 0;

  const currentSquares = history[currentMove];

  function handlePlay(nextSquares) {
    // Set the history to be all previous history elements, plus the new state.
    const nextHistory = [...history.slice(0, currentMove + 1), nextSquares];
    setHistory(nextHistory);
    setCurrentMove(nextHistory.length - 1);
  }

  // Perform some time travel!
  function timeTravel(nextMove) {
    setCurrentMove(nextMove);
  }

  // Build up a list of our previous moves
  const moves = history.map((squares, move) => {
    let description;
    if (move > 0) {
      description = "Go to move #" + move;
    } else {
      description = "Go to game start";
    }
    if (move == currentMove) {
      return (<p>You are on move {move}</p>)
    } else {
      return (
        <li key={move}>
          <button onClick={() => timeTravel(move)}>{description}</button>
        </li>
      )
    }
  })

  return (
    <div className="game">
      <div className="game-board">
        <Board xIsNext={xIsNext} squares={currentSquares} onPlay={handlePlay} />
      </div>
      <div className="game-info">
        <ol>{moves}</ol>
      </div>
    </div>
  )
}

// This is the "main" function of the file. This renders the game board,
// and handles state.
function Board({ xIsNext, squares, onPlay }) {
  // When a square is clicked.
  function handleClick(i: number) {
    // Calculate a winner. If we have one, exit this function early.
    if (calculateWinner(squares)) {
      return;
    }
    // Check if this space is already filled. If it is we cant fill it.
    if (squares[i]) {
      return;
    }

    // Create an immutable view over our squares array
    const nextSquares = squares.slice();
    // Check the player state. If it's X then mutate the nextSquares array
    // to contain X, otherwise O.
    if (xIsNext) {
      nextSquares[i] = 'X';
    } else {
      nextSquares[i] = 'O';
    }

    onPlay(nextSquares);
    // Update the state of the squares array from the enclosing scope.
    //setSquares(nextSquares);
    // Switch player.
    //setXIsNext(!xIsNext);
  }

  const winner = calculateWinner(squares);
  let status;
  if (winner) {
    status = "Winner: " + winner;
  } else {
    status = "Next player is: " + (xIsNext ? 'X' : 'O');
  }

  return (
    <>
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
    </>
  );
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
    [2, 4, 6],
  ];
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return squares[a];
    }
  }
  return null;
}

export default Game;

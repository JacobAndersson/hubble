import { useEffect, useState, useRef } from 'react';
import Chess from 'chess.js';
import { Chessboard } from 'react-chessboard';
import MovePicker from './MovePicker';
import styles from './Board.module.css';

export default function Board({moves, game, blunders, moveIdx, onMoveChange, onKeyPress}) {
  const moveIdxRef = useRef(moveIdx);

  function handleKeyPress(e) {
    if (moves.length === 0) {
      return;
    }

    let code = e.code;
    if (code === 'ArrowRight') {
      onKeyPress(1)
    } else if (code === 'ArrowLeft') {
      onKeyPress(-1);
    }
  }

  function handleMoveClick(_, idx) {
    onMoveChange(idx);
  }

  useEffect(() => {
    let listener = document.addEventListener('keydown', handleKeyPress);
    return () => {
      document.removeEventListener('keydown', listener);
    }
  }, []);

  return (
    <div className={styles.container}>
      <Chessboard position={game.fen()} />
      <MovePicker moves={moves} onMoveClick={handleMoveClick} blunders={blunders} currentIdx={moveIdx} />
    </div>
  );
}

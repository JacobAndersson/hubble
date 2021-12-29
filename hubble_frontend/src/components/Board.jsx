import { useEffect, useState, useRef } from 'react';
import Chess from 'chess.js';
import { Chessboard } from 'react-chessboard';
import MovePicker from './MovePicker';
import styles from './Board.module.css';

export default function Board({moves}) {
  const [game, setGame] = useState(new Chess());
  const moveIdx = useRef(0);

  function safeGameMutate(modify) {
    setGame((g) => {
      const update = { ...g };
      modify(update);
      return update;
    });
  }

  function handleKeyPress(e) {
    if (moves.length === 0) {
      return;
    }

    let code = e.code;
    if (code === 'ArrowRight' && moveIdx.current < moves.length) {
      safeGameMutate(game => {
        let mv = moves[moveIdx.current];
        let src = mv.slice(0, 2);
        let target = mv.slice(2, 4);

        game.move({
          from: src,
          to: target,
          promotion: 'q' // always promote to a queen for example simplicity
        });
        moveIdx.current += 1;
      });
    } else if (code === 'ArrowLeft' && moveIdx.current > 0) {
      safeGameMutate(game => {
        game.undo();
        moveIdx.current -= 1;
      })
    }
  }

  function handleMoveClick(_, idx) {
    let diff = idx - moveIdx.current;

    if (diff >= 0) {
      safeGameMutate(game => {
        for (let i = 0; i <= diff; i++) {
          let mv = moves[moveIdx.current];
          let src = mv.slice(0, 2);
          let target = mv.slice(2, 4);

          game.move({
            from: src,
            to: target,
            promotion: 'q' //TODO: FIXE PROMOTION
          });
          moveIdx.current += 1;
        }
          moveIdx.current -= 1;
      });
    } else if (diff < 0) {
        safeGameMutate(game => {
          for (let i = 0; i > diff; i --) {
            game.undo();
            moveIdx.current -= 1;
          }
      });
    } 
  }

  function onDrop(sourceSquare, targetSquare) {
    let move = null;
    safeGameMutate((game) => {
      let mv = moves[moveIdx.current];
      
      let src = mv.slice(0, 2);
      let target = mv.slice(2, 4);
      move = game.move({
        from: src,
        to: target,
        promotion: 'q' // always promote to a queen for example simplicity
      });
    });
    moveIdx.current += 1;
  }

  useEffect(() => {
    let listener = document.addEventListener('keydown', handleKeyPress);
    return () => {
      document.removeEventListener('keydown', listener);
    }
  }, []);

  return (
    <div className={styles.container}>
      <Chessboard position={game.fen()} onPieceDrop={onDrop} />
      <MovePicker moves={moves} onMoveClick={handleMoveClick} />
    </div>
  );
}

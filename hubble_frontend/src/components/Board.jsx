import { useEffect, useState } from 'react';
import Chess from 'chess.js';
import { Chessboard } from 'react-chessboard';

export default function Board({moves}) {
  const [game, setGame] = useState(new Chess());
  let moveIdx = 0; 

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
    if (code === 'ArrowRight' && moveIdx < moves.length) {
      safeGameMutate(game => {
        console.log(moves[moveIdx]);
        game.move(moves[moveIdx]);
        moveIdx += 1;
      });
    } else if (code === 'ArrowLeft' && moveIdx > 0) {
      safeGameMutate(game => {
        game.undo();
        moveIdx -= 1;
      })
    }
  }

  useEffect(() => {
    let listener = document.addEventListener('keydown', handleKeyPress);
    return () => {
      document.removeEventListener('keydown', listener);
    }
  }, []);

  return <Chessboard position={game.fen()} />;
}

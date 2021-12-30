import { useState, useEffect, useRef} from 'react';
import axios from 'axios';
import { useParams } from "react-router-dom";
import Chess from 'chess.js';

import Board from './Board';
import ScoreChart from './ScoreChart';

import styles from './Analysis.module.css';

export default function Analysis() {
  let params = useParams();
  let gameId = params?.id;

  const [game, setGame] = useState({});
  const [blunders, setBlunders] = useState([]);
  const [board, setBoard] = useState(new Chess());

  const [moveIdx, _setMoveIdx] = useState(-1);
  const moveIdxRef = useRef(-1);

  const setMoveIdx = newIdx => {
    _setMoveIdx(newIdx);
    moveIdxRef.current = newIdx;
  }

  useEffect(() => {
    getScores().then( (newGame) => {
      setGame(newGame);
    })

    getBlunders().then(data => {
      setBlunders(data);
    });
  }, [gameId])

  function getScores() {
    return axios.get(`/api/analyse/match/${gameId}`).then(res => {
      return res?.data
    });
  }

  function getBlunders() {
    return axios.get(`/api/blunder/${gameId}`).then(res => {
      return res?.data;
    });
  }

  function safeGameMutate(modify, done) {
    setBoard((g) => {
      const update = { ...g };
      modify(update);
      return update;
    });
  }

  function handleMoveChange(newIdx) {
    let diff = newIdx - moveIdx;

    if (diff >= 0 ){
      safeGameMutate(board => {
        for (let i = 0; i < diff; i++) {
          let mv = game.moves[moveIdx + i + 1];
          let src = mv.slice(0, 2);
          let target = mv.slice(2, 4);

          board.move({
            from: src,
            to: target,
            promotion: 'q' //TODO: FIXE PROMOTION
          });
        }
      });
    } else {
      safeGameMutate(board => {
        for (let i = 0; i > diff; i --) {
          board.undo();
        }
      });
    }
    setMoveIdx(newIdx);
  }
 
  function onKeyPress(diff) {
    handleMoveChange(moveIdxRef.current + diff);
  }

  if (Object.keys(game).length === 0){
    return (
      <div>
        <p>{gameId}</p>
      </div>
    );
  } else {
    return (
      <div className={styles.container}>
        <div>
          <p>{gameId}</p>
          <p>{`${game.black}-${game.black_rating}`}</p>
          <Board game={board} moves={game.moves} blunders={blunders} moveIdx={moveIdx} onMoveChange={handleMoveChange} onKeyPress={onKeyPress} />
          <p>{`${game.white}-${game.white_rating}`}</p>
        </div>
        <ScoreChart game={game} moveIdx={moveIdx} /> 
      </div>
    );
  }
}

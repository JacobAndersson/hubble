import { useState, useEffect } from 'react';
import Chess from 'chess.js';
import { Chessboard } from 'react-chessboard';
import axios from 'axios';

import { Line } from 'react-chartjs-2';
import styles from './Analysis.module.css';

import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

export default function Analysis({ gameId }) {
	const [game, setGame] = useState(new Chess());
  const [white, setWhite] = useState("");
  const [black, setBlack] = useState("");

  const [dataset, setDataset] = useState({});

  useEffect(() => {
    getScores().then( ({ newWhite, newBlack, newScores }) => {
      setWhite(newWhite);
      setBlack(newBlack);

      setDataset({
        labels: newScores.map((_, i)=> `move ${i}`),
        datasets: [{
          label: "Score",
          data: newScores,
          tension: 0.3,
        }]
      });
    })
  }, [gameId])

  function getScores() {
    /*
    return new Promise((resolve, reject) => {
        resolve({
            newWhite: "jrti",
            newBlack: "Pavel1511",
            newScores: [ 
              -25, 40, 0, 845, -15, 22, -36, 27, 72, 62, 46,
              58, 33, 54, 55, -63, -55, -47, -35, -72, -50, 107,
              99, 90, 99, 111, -93, 99, -98, 160, -150, 148, -150,
              250, 242, 234, 221, 203, 217, 220, -152, -160, -152,
              -154, 150, 159, 165, -170, 210 
            ]
        })
    });
    */
    return axios.get(`/api/analyse/${gameId}`).then(res => {
      const { white: newWhite, black: newBlack, scores: newScores } = res?.data
      return { newWhite, newBlack, newScores };
    });
  }

  function safeGameMutate(modify) {
    setGame((g) => {
      const update = { ...g };
      modify(update);
      return update;
    });
  }

  function makeRandomMove() {
    const possibleMoves = game.moves();
    if (game.game_over() || game.in_draw() || possibleMoves.length === 0) return; // exit if the game is over
    const randomIndex = Math.floor(Math.random() * possibleMoves.length);
    safeGameMutate((game) => {
      game.move(possibleMoves[randomIndex]);
    });
  }

  function onDrop(sourceSquare, targetSquare) {
    let move = null;
    safeGameMutate((game) => {
      move = game.move({
        from: sourceSquare,
        to: targetSquare,
        promotion: 'q' // always promote to a queen for example simplicity
      });
    });

    if (move === null) return false; // illegal move
    setTimeout(makeRandomMove, 200);
    return true;
  }

  if (Object.keys(dataset).length === 0){
    return (
      <div>
        <p>WAIT</p>
      </div>
    );
  } else {
    return (
      <div className={styles.container}>
        <div>
          <p>{`${black.name}-${black.rating}`}</p>
          <Chessboard position={game.fen()} onPieceDrop={onDrop} />
          <p>{`${white.name}-${white.rating}`}</p>
        </div>
        <div className = {styles.chartContainer}>
          <Line
            data={dataset}
            options={{
              responsive: true,
              maintainAspectRatio: false,
            }}
          />
        </div>
      </div>
    );
  }
}

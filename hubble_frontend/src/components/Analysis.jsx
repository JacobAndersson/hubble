import { useState, useEffect } from 'react';
import axios from 'axios';
import Board from './Board';

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
  const [dataset, setDataset] = useState({});
  const [game, setGame] = useState({});

  useEffect(() => {
    getScores().then( (newGame) => {
      setGame(newGame);

      setDataset({
        labels: newGame.scores.map((_, i)=> `move ${i}`),
        datasets: [{
          label: "Score",
          data: newGame.scores,
          tension: 0.3,
        }]
      });
    })
  }, [gameId])

  function getScores() {
    return axios.get(`/api/analyse/match/${gameId}`).then(res => {
      console.log(res.data);
      return res?.data
    });
  }

  if (Object.keys(dataset).length === 0){
    return (
      <div>
        <p>{gameId}</p>
        <p>WAIT</p>
      </div>
    );
  } else {
    return (
      <div className={styles.container}>
        <div>
          <p>{gameId}</p>
          <p>{`${game.black}-${game.black_rating}`}</p>
          <Board moves={game.moves} />
          <p>{`${game.white}-${game.white_rating}`}</p>
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
        {JSON.stringify(game.moves)}
      </div>
    );
  }
}

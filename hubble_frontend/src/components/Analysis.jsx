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
	
  const [white, setWhite] = useState("");
  const [black, setBlack] = useState("");
  const [dataset, setDataset] = useState({});
  const [moves, setMoves] = useState([]);

  useEffect(() => {
    getScores().then( ({ white: newWhite, black: newBlack, scores: newScores, moves: newMoves}) => {
      setWhite(newWhite);
      setBlack(newBlack);
      setMoves(newMoves);

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
    return axios.get(`/api/analyse/match/${gameId}`).then(res => {
      console.log(res.data);
      return res?.data
    });
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
          <Board moves={moves} />
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
        {JSON.stringify(moves)}
      </div>
    );
  }
}

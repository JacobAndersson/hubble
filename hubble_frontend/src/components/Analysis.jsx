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
    return new Promise((resolve, reject) => {
        resolve({"white":{"name":"jrti","rating":2073},"black":{"name":"Cunastyle","rating":1986},"scores":[-41.0,111.0,29.0,101.0,-34.0,83.0,-1.0,32.0,72.0,61.0,-67.0,156.0,-141.0,145.0,-144.0,146.0,-132.0,179.0,-81.0,166.0,-154.0,209.0,-224.0,233.0,-130.0,157.0,149.0,141.0,149.0,167.0,128.0],"moves":["d4","c5","c4","Nf6","Nf3","e6","g3","cxd4","Bg2","Nc6","Nxd4","Nxd4","Qxd4","Qc7","Nc3","Bc5","Qd3","O-O","b3","Rd8","O-O","Rb8","Bf4","d6","Nb5","Qb6","e4","e5","Bg5","Ne8","Bxd8"]})
    });
    /*
    return axios.get(`/api/analyse/${gameId}`).then(res => {
      console.log(res.data);
      return res?.data
    });
    */
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

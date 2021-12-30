import React from "react";
import { Line } from "react-chartjs-2";
import annotationPlugin from "chartjs-plugin-annotation";

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";

import styles from "./ScoreChart.module.css";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  annotationPlugin
);

export default function ScoreChart({ game, moveIdx }) {
  let dataset = {
    labels: game?.scores.map((_, i) => `move ${i}`),
    datasets: [
      {
        label: "Score",
        data: game?.scores,
        tension: 0.3,
      },
    ],
  };

  return (
    <div className={styles.container}>
      <Line
        data={dataset}
        options={{
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            autocolors: false,
            annotation: {
              annotations: {
                moveLine: {
                  type: "line",
                  xMin: moveIdx || 0,
                  xMax: moveIdx || 0,
                  borderColor: "rgba(255, 99, 132)",
                  borderWidth: 2,
                },
                zero: {
                  type: "line",
                  yMin: 0,
                  yMax: 0,
                  borderColor: "rgba(128, 128, 128)",
                  borderWidth: 2,
                },
              },
            },
          },
        }}
      />
    </div>
  );
}

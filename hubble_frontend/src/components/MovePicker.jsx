import styles from "./MovePicker.module.css";
import classnames from "classnames";

function pairs(arr) {
  return arr.reduce((result, value, index, array) => {
    if (index % 2 === 0) {
      result.push(array.slice(index, index + 2));
    } else if (index === arr.length - 1) {
      result.push([value]);
    }
    return result;
  }, []);
}

export default function MovePicker({
  moves,
  blunders,
  onMoveClick,
  currentIdx,
}) {
  let blundersIdx = blunders.map((x) => x[0]);

  let mvs = pairs(moves).map((mv, idx) => {
    return (
      <div className={styles.moveContainer} key={`${mv}-${idx}`}>
        {mv.map((m, i) => {
          let mvIdx = 2 * idx + i;
          let isBlunder = blundersIdx.includes(mvIdx);
          let isCurrent = mvIdx === currentIdx;

          return (
            <p
              key={`${m}-${i}`}
              onClick={() => onMoveClick(m, mvIdx)}
              className={classnames(
                styles.move,
                isBlunder ? styles.blunder : "",
                isCurrent ? styles.current : ""
              )}
            >
              {m}
            </p>
          );
        })}
      </div>
    );
  });

  return <div className={styles.container}>{mvs}</div>;
}

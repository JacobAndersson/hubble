import styles from './MovePicker.module.css';

function pairs(arr) {
  return arr.reduce((result, value, index, array) => {
    if (index % 2 === 0){
        result.push(array.slice(index, index + 2));
    } else if (index == arr.length - 1) {
      result.push([value])
    }
    return result;
  }, []);
}

export default function MovePicker({ moves, onMoveClick}) {
  let mvs  = pairs(moves).map((mv, idx) => {
    return (
      <div className={styles.moveContainer} key={`${mv}-${idx}`}>
        {mv.map((m, i) => {
          return (
            <p key={`${m}-${i}`} onClick={() => onMoveClick(m, 2*idx + i)} className={styles.move}>{m}</p>
          );
        })}
      </div>
    );
  });

  return (
    <div className={styles.container}>
      {mvs}
    </div>
  );
}

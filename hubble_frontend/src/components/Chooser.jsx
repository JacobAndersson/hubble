import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Chooser() {
  const [games, setGames] = useState([]);
  useEffect(() => {
    getGames(); 
  }, [])

  function getGames() {
    axios.get('/api/games').then(res => {
      setGames(res.data);
    });
  }

  return (
    <>
      {games.map(x => {
        return (
          <div key={x.id}>
            <a href={`/analyse/${x.id}`}>{`${x.id}`}</a>
          </div>
        );
      })}
    </>
  )

}

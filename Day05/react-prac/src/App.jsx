import React, { useState } from 'react';

const App = () => {
  const [user, setUser] = useState(null); // State to store user data

  const fetchUser = () => {
    fetch("https://randomuser.me/api/")
      .then((response) => {
        return response.json(); // Call json() to parse the response
      })
      .then((data) => {
        // Set user data to state
        setUser(data.results[0]); // Assuming we want the first result
      })
      .catch((error) => {
        console.error('Error fetching data:', error); // Handle any errors
      });
  };

  return (
    <div>
      <h1>Hello ji</h1>
      <button onClick={fetchUser}>Generate New User</button> {/* Button to fetch new user */}
      {user && ( // Conditionally render user info if user data exists
        <div>
          <h4>Name : {`${user.name.first} ${user.name.last}`}</h4> {/* Display user's name */}
          <p>Email: {user.email}</p> {/* Display user's email */}
          <img src={user.picture.large} alt={`${user.name.first} ${user.name.last}`} /> {/* Display user's image */}
        </div>
      )}
    </div>
  );
};

export default App;

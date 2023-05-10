window.array_of_flights = [];

function generateObjects() {
  const objects = [];
  const latitudeRange = [-70, 70];
  const longitudeRange = [-180, 180];
  const objectCount = 1000;

  for (let i = 1; i <= objectCount; i++) {
    const latitude = getRandomNumber(latitudeRange[0], latitudeRange[1]);
    const longitude = getRandomNumber(longitudeRange[0], longitudeRange[1]);
    const name = `Object_${i}`;
    const direction = getRandomDirection();
    objects.push({ latitude, longitude, name, direction });
  }

  return objects;
}

function moveObjects(objects) {
  const speed = 0.00125;
  const moveInterval = setInterval(() => {
    objects.forEach(object => {
      const latChange = object.direction[0] * speed;
      const lngChange = object.direction[1] * speed;
      object.latitude += latChange;
      object.longitude += lngChange;
    });

    // Print the updated positions (for demonstration purposes)
    objects.forEach(object => {
      window.array_of_flights.push(`${object.name};${object.longitude.toFixed(6)};${object.latitude.toFixed(6)};false`);
    });
  }, 16); // Change the interval duration (in milliseconds) as needed

  // Stop the interval after a specific duration (e.g., 5 minutes)
  setTimeout(() => {
    clearInterval(moveInterval);
  }, 5 * 60 * 1000);
}

function getRandomNumber(min, max) {
  return Math.random() * (max - min) + min;
}

function getRandomDirection() {
  // Generate a random angle between 0 and 2*pi radians (360 degrees)
  const angle = Math.random() * 2 * Math.PI;

  // Convert the angle to a unit vector for direction
  const x = Math.cos(angle);
  const y = Math.sin(angle);

  return [x, y];
}

// Example usage:
const objects = generateObjects();
moveObjects(objects);

export function get_flight() {
  return window.array_of_flights.shift();
}
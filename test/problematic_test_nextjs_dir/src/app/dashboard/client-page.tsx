'use client'

export default function Dashboard() {
  // Data fetching that could be moved to server
  const getData = async () => {
    const res = await fetch('/api/data');
    return res.json();
  };

  // Heavy computation that could be server-side
  const processUserData = (data) => {
    return data.map(user => {
      // Complex transformation
      return transform(user);
    });
  };

  return <div>Dashboard</div>;
}

function transform(data) {
  // Heavy computation
  return data;
}

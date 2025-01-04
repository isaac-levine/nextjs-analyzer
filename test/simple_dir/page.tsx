'use client'
export default function Dashboard() {
  const getData = async () => {
    const res = await fetch('/api/data')
    return res.json()
  }
  return <div>Dashboard</div>
}

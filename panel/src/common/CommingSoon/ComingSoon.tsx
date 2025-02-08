import "./ComingSoon.css"

export interface ComingSoonProps {
    children?: React.ReactNode
}

export default function ComingSoon({children}: ComingSoonProps) {
  return <div className="coming-soon">
    <span className="overlayText">Coming Soon!</span>
    {children}
  </div>
}
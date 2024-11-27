import "./Placeholder.css"

interface PlaceholderProps {
  bottomText?: string
}

export default function Placeholder({bottomText}: PlaceholderProps) {
  return <div className="placeholder">
    <div className="titleText">PLACEHOLDER</div>
    {bottomText ?
      <div className="bottomText">{bottomText}</div> : <></>
    }
  </div>
}
import {Button} from "@shadcn/button";
import "./DisruptiveActionPopout.css"
import IconWarningS from "@i/IconWarningS.tsx";
import {useComponentOverlay} from "@s/popoutProvider/PopoutProvider.tsx";

export interface DisruptiveActionPopupProps {
  warnings: string[]
  onCancel?: () => void;
  onConfirm: () => void;
}

export default function DisruptiveActionPopup({warnings, onConfirm, onCancel}: DisruptiveActionPopupProps) {
  const {clearComponent} = useComponentOverlay();
  // const warnings: string[] = [
  //   "test message", "test readon 2",
  //   "test message", "test readon 2",
  // ];
  return <div className="disruptiveActionPopup">
    <div className="warningIconBox">
      <IconWarningS/>
    </div>
    <div className="body">
      <div className="title">
        <h1>This action may be <b>disruptive</b> or <b>confusing</b>!</h1>
        <h2>Please confirm you want to proceed with the following actions:</h2>
      </div>
      <div className="darkBox">
        <div className="causes">
        {warnings?.map((warning, i) => (<div key={i}>- {warning}</div>))}
        </div>
        <div className="buttons">
          <Button onClick={() => {
            onCancel && onCancel();
            clearComponent();
          }}>Cancel</Button>
          <Button onClick={() => {
            onConfirm();
            clearComponent();
          }}>Confirm</Button>
        </div>
      </div>
    </div>
  </div>
}
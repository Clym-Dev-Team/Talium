import {useCallback, useState} from "react";
import "./TicketCard.css"
import {useForm} from "react-hook-form";
import {useToast} from "@shadcn/use-toast.ts";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import {fetchWithAuth} from "@/components/Login/LoginPage.tsx";
import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";
import IconX from "@/assets/IconX.tsx";
import IconSave from "@/assets/IconSave.tsx";
import IconCheck from "@/assets/IconCheck.tsx";

export interface WinnerCardProps {
  username: string,
  userId: string,
}

interface WinnerState {
  userId: string,
  comment?: string,
  rejected: boolean,
}

export default function WinnerCard({userId, username}: WinnerCardProps) {
  const { toast } = useToast();
  const [isCommentEdit, setIsCommentEdit] = useState(false)
  const {register, getValues, setValue, formState, reset, handleSubmit} = useForm<WinnerState>({
    defaultValues: {
      comment: undefined,
      rejected: false,
      userId: userId,
    }
  });

  const submit = useCallback((state: WinnerState) => {
    //TODO make endpoint                 \/
    fetchWithAuth("/!TODO!", {method: "POST", body: JSON.stringify(state)})
      .then(() => toast({className: "toast toast-success", title: "Saved Winner!"}))
      .catch(reason => toast({
        className: "toast toast-failure",
        title: "ERROR saving winner information",
        description: reason.toString()
      }))
    reset(state);
  }, [])

  return <div className={getValues("rejected") ? "rejected gwResultCard winnerCard" : "gwResultCard winnerCard"} onClick={() => setIsCommentEdit(true)}>
    <div className="winnerData">
      <span className="username">{username}</span>
      {isCommentEdit ?
        <Input type="text" className="comment" {...register("comment")} autoFocus={true}/> :
        <span className="comment">{getValues("comment")}</span>
      }
    </div>
    <div className="actions">
      {formState.isDirty ? <Button type="submit" onClick={handleSubmit(submit)}><IconSave/></Button> : ""}
      <IconCheckBox checked={getValues("rejected")} onChange={b => {setValue("rejected", b, { shouldDirty: true }); setIsCommentEdit(true)}} icon={<IconX/>}
                    checkedIcon={<IconCheck/>}/>
    </div>
  </div>
}
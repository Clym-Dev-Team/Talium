import {useToast} from "@shadcn/use-toast.ts";
import {useCallback} from "react";
import {fetchWithAuth} from "@/components/Login/LoginPage.tsx";

export default function useFetch() {
  const {toast} = useToast();
  const fetch = useCallback((urlPath: string, errorToast: string, successToast?: string, init?: RequestInit) => {
    return fetchWithAuth(urlPath, init)
      .then(response => response.json())
      .then(value => {
        if (successToast) {
          toast({className: "toast toast-success", title: successToast});
        }
        return value;
      })
      .catch(reason => toast({
        className: "toast toast-failure",
        title: errorToast,
        description: reason.toString()
      }))
  }, [toast]);

  return {sendData: fetch};
}

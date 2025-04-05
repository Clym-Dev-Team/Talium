import {useCallback, useEffect, useState} from "react";
import {useToast} from "@shadcn/use-toast.ts";
import {fetchWithAuth} from "@/components/Login/LoginPage.tsx";

export default function useData<T>(urlPath: string, objectName: string, initialValue: T, init?: RequestInit) {
  const {toast} = useToast();
  const [data, setData] = useState<T>(initialValue);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false)

  const get = useCallback(() => {
    if (error) {
      return;
    }
    let ignore = false;
    fetchWithAuth(urlPath, init).then()
      .then(response => response.json())
      .then(json => {
        if (!ignore) {
          setData(json);
          setLoading(false);
        }
      })
      .catch(reason => toast({
        className: "toast toast-failure",
        title: "ERROR loading " + objectName,
        description: reason.toString()
      }))
      .catch(() => setLoading(false))
      .catch(() => setError(true))
    return () => {
      ignore = true;
    };

  }, [error, init, objectName, toast, urlPath]);

  useEffect(() => {
    get()
  }, [get]);

  const sendData = useCallback((urlPath: string, successToast: string, init?: RequestInit) => {
    return fetchWithAuth(urlPath, init)
      .then(response => response.json())
      .then(value => {
        toast({className: "toast toast-success", title: successToast});
        get();
        return value;
      })
      .catch(reason => toast({
        className: "toast toast-failure",
        title: "ERROR saving " + objectName,
        description: reason.toString()
      }))
  }, [objectName, toast]);

  return {data, loading, sendData};
}
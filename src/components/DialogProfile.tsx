import { Button, Dialog, DialogActions, DialogContent, DialogTitle, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Fragment } from "react/jsx-runtime";

export interface User {
    name: string,
    email: string
}

export default function DialogProfile(props: any) {
    const [data, setData] = useState<User | null>(null);

    async function featchUser() {
        try {
            const fetch = await invoke<User>('get_user_profile');
            setData(fetch);
        } catch(err) {
            console.log(err)
        }
    }
    featchUser();
        
    const handleClose = () => {
        props.open(false);
    }

    return (
        <Fragment>
            <Dialog open={true} maxWidth='sm' fullWidth={true}>
                <DialogTitle>Informações</DialogTitle>
                <DialogContent>
                    <Typography>Nome: {data?.name}</Typography>
                    <Typography>Email: {data?.email}</Typography>
                    <DialogActions>
                        <Button onClick={handleClose} autoFocus>Fechar</Button>
                    </DialogActions>
                </DialogContent>
            </Dialog>
        </Fragment>
    )
}
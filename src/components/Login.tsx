import { Avatar, Box, Button, Container, LinearProgress, Paper, TextField, Typography } from "@mui/material";
import FilterDramaIcon from '@mui/icons-material/FilterDrama';
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function Login(props: any) {
    const [user, setUser] = useState("");
    const [password, setPassword] = useState("");
    const [textMessage, setTextMessage] = useState("");

    const [load, setLoad] = useState(false);
    const [errInput, setErrInput] = useState(false);

    function handleSubmit(event: any) {
        event.preventDefault();

        setErrInput(false)
        setLoad(true);
        invoke('login', { user: user, password: password })
            .then((message) => isLogin(message))
            .catch((error) => console.log(error));
    }

    function isLogin(message: any) {
        if(message === 200) {
            props.onStatus(message)
        } else if(message === 401) {
            setLoad(false)
            setErrInput(true)
            setTextMessage("Usuário ou senha incoretos")
        } else {
            setErrInput(true)
            setTextMessage("Erro na requisição")
        }
    }

    return (
        <Container maxWidth="xs">
            <Paper elevation={10} sx={{marginTop: 8, padding: 2}}>
                <Avatar sx={{mx: "auto", bgcolor: "primary.main", textAlign: "center", mb: 1}}>
                    <FilterDramaIcon />
                </Avatar>
                <Typography component="h1" variant="h5" sx={{ textAlign: "center"}}>
                    ClimaCampo
                </Typography>
                <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
                    <TextField placeholder="Usuario" fullWidth required autoFocus type="text" sx={{ mb: 2 }} onChange={(e) => setUser(e.target.value)}/>
                    <TextField placeholder="Senha" fullWidth required autoFocus type="password" sx={{ mb: 2 }} onChange={(e) => setPassword(e.target.value)}/>
                    {load && <LinearProgress />}
                    {errInput && <Typography sx={{ textAlign: "center" }} variant="overline" color="error">{textMessage}</Typography>}
                    <Button type="submit" variant="contained" fullWidth sx={{ mt: 1 }}>Entrar</Button>
                </Box>
            </Paper>
        </Container>
    )
}


import { Avatar, Box, Button, Container, LinearProgress, Paper, TextField, Typography } from "@mui/material";
import FilterDramaIcon from '@mui/icons-material/FilterDrama';
import { useEffect, useState } from "react";


export function Login(props: any) {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    
    const [load, setLoad] = useState(false);
    const [errInput, setErrInput] = useState(false);

    function handleSubmit(event: any) {
        event.preventDefault();
        setErrInput(false);
        setLoad(true);
        props.LoginForm({username, password})
    }

    useEffect(() => {
        if(props.error) {
            setErrInput(true);
            setLoad(false);
        }
    }, [props.error])
    
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
                    <TextField placeholder="Usuario" fullWidth required autoFocus type="text" sx={{ mb: 2 }} onChange={(e) => setUsername(e.target.value)}/>
                    <TextField placeholder="Senha" fullWidth required type="password" sx={{ mb: 2 }} onChange={(e) => setPassword(e.target.value)}/>
                    {load && <LinearProgress />}
                    {errInput && <Typography sx={{ textAlign: "center" }} variant="overline" color="error">{props.error}</Typography>}
                    <Button type="submit" variant="contained" fullWidth sx={{ mt: 1 }}>Entrar</Button>
                </Box>
            </Paper>
        </Container>
    )
}


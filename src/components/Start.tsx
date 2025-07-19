import { Box, Divider, Grid, Paper, Typography } from "@mui/material";

const cardClimate = [{
    id: 0,
    farm: 'Fazenda Água Branca',
    temperature: {
        current: 28,
        max: 32,
        average: 26,
        min: 21
    },
    humidity: {
        max: 95,
        average: 80,
        min: 75
    },
    wind: {
        max: 20,
        average: 12,
        direction: 'Norte'
    },
}, {
    id: 1,
    farm: 'Fazenda Esperança',
    temperature: {
        current: 27,
        max: 30,
        average: 24,
        min: 18
    },
    humidity: {
        max: 96,
        average: 83,
        min: 78
    },
    wind: {
        max: 21,
        average: 10,
        direction: 'Leste'
    },
}, {
    id: 2,
    farm: 'Fazenda Morada do Sol',
    temperature: {
        current: 29,
        max: 35,
        average: 28,
        min: 23
    },
    humidity: {
        max: 90,
        average: 74,
        min: 70
    },
    wind: {
        max: 27,
        average: 15,
        direction: 'Oeste'
    },
}]

export function Start() {

    return (
        <Box sx={{ flexGrow: 1 }}>
            <Typography variant="h5">Indicativos atuais</Typography>
            <Divider sx={{marginTop: '0.25rem', marginBottom: '1rem'}}/>
            <Grid container spacing={2}>
                {cardClimate.map((data) => (
                    <Paper key={data.id} elevation={10} sx={{ width: '21.8rem', height: '20rem', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1rem',padding: '1rem' }}>
                        <Typography variant="h6">{data.farm}</Typography>
                        <Typography variant="h3">{data.temperature.current}°C</Typography>
                        <Grid>
                            <Typography variant="subtitle2">Max: {data.temperature.max}°C &nbsp; &nbsp; Min: {data.temperature.min}°C</Typography>
                        </Grid>
                        <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }} sx={{width: '100%', justifyContent: 'center'}}>
                            <Paper sx={{width: '45%', display: 'flex', flexDirection: 'column',justifyContent: 'center', padding: '0.7rem', gap: '0.2rem'}}>
                                <Typography  variant="subtitle1">Humidade</Typography>
                                <Typography variant="subtitle2">Altual: {data.humidity.average}%</Typography>
                                <Typography variant="subtitle2">Max: {data.humidity.max}%</Typography>
                                <Typography variant="subtitle2">Min: {data.humidity.min}%</Typography>
                            </Paper>
                            <Paper sx={{width: '45%', display: 'flex', flexDirection: 'column',justifyContent: 'center', padding: '0.7rem', gap: '0.2rem'}}>
                                <Typography  variant="subtitle1">Vel. do vento</Typography>
                                <Typography variant="subtitle2">Atual: {data.wind.average} km</Typography>
                                <Typography variant="subtitle2">Rajada: {data.wind.max} km</Typography>
                                <Typography variant="subtitle2">Direção: {data.wind.direction}</Typography>
                            </Paper>
                        </Grid>
                    </Paper>
                ))}
            </Grid>
        </Box>
    )
}
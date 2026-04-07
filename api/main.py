from fastapi import FastAPI
from api.routes import proposals, votes, milestones, participants
from dotenv import load_dotenv

load_dotenv()

app = FastAPI(title="ClinicalDAO API", version="0.1.0")

app.include_router(proposals.router)
app.include_router(votes.router)
app.include_router(milestones.router)
app.include_router(participants.router)


@app.get("/health")
def health():
    return {"status": "ok"}

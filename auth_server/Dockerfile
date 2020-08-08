FROM python

WORKDIR /app

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY src/ .

CMD [ "uvicorn", "app:app", "--reload" ]

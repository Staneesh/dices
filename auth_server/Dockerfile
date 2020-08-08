FROM python

WORKDIR /app

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY src/ .

EXPOSE 8000
CMD [ "uvicorn", "app:app", "--reload", "--host", "0.0.0.0" ]

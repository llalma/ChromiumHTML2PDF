import asyncio
import html2pdf
import os

HTML_FILE_TO_CONVERT = f"File://{os.path.join(os.getcwd(),'test.html')}"
OUTPUT_PATH = 'output.pdf'

CHROME_PATH = 'test chrome path'

async def main():
    await html2pdf.convert(
            input_path=HTML_FILE_TO_CONVERT,
            output_path=OUTPUT_PATH,
            chromeium_path=CHROME_PATH
        )

asyncio.run(main())